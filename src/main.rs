use actix_web::{
    web, App, Error, Responder,
    HttpRequest, HttpServer, HttpResponse
};
use actix_web::web::Data;
use eventstore::{All, Client, EventData, ReadResult};
use futures::stream::TryStreamExt;
use serde::{Serialize, Deserialize};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use actix_files as fs;
use std::time::{Duration, Instant};
use actix::prelude::*;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(30);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Serialize, Deserialize, Debug)]
struct Greeting {
    name: String,
}

async fn greet(db: web::Data<Client>, req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");

    let payload = Greeting {
        name: name.to_string(),
    };
    let greet = EventData::json("greeting", &payload).unwrap();

    let _ = db
        .append_to_stream("greeting-stream", &Default::default(), greet)
        .await.unwrap();

    format!("Hello {}!", &name)
}

/// Define HTTP actor
struct MyWs{
    heart_beat: Instant,
}

impl Actor for MyWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heart_beat(ctx);
    }
}


impl MyWs {
    fn new(db: web::Data<Client>) -> Self {
        Self {
            heart_beat: Instant::now(),
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn heart_beat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.heart_beat) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }

}

/// Handler for ws::Message message
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWs {

    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg)
            },
            Ok(ws::Message::Pong(_)) => {
                self.heart_beat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

async fn ws(db: web::Data<Client>,req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    println!("{:?}", req);
    let resp = ws::start(MyWs::new(db), &req, stream);
    println!("{:?}", resp);
    resp
}


#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false".parse()?;
    let client = Client::create(settings).await?;


    let result = client
        .read_stream("greeting-stream", &Default::default(), All)
        .await?;

    if let ReadResult::Ok(mut stream) = result {
        while let Some(event) = stream.try_next().await? {
            let event = event.get_original_event()
                .as_json::<Greeting>()?;

            // Do something productive with the result.
            println!("{:?}", event);
        }
    }

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(client.clone()))
            .route("/ws", web::get().to(ws))
            .route("/greeting/", web::get().to(greet))
            .route("/greeting/{name}", web::get().to(greet))
            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;

    Ok(())
}