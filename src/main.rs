#[macro_use]
extern crate rocket;

use eventstore::{Client, EventData, SubEvent};
use rocket::fs::{relative, FileServer};
use rocket::futures::TryStreamExt;
use rocket::response::stream::{Event, EventStream};
use rocket::serde::{Deserialize, Serialize};
use rocket::State;

#[derive(Serialize, Deserialize, Debug)]
#[serde(crate = "rocket::serde")]
struct Greeting {
    name: String,
}

struct DbState {
    db: Client,
}

#[get("/<name>")]
async fn greet(db_state: &State<DbState>, name: &str) -> String {
    let db = db_state.db.clone();

    let payload = Greeting {
        name: name.to_string(),
    };
    let greet = EventData::json("greeting", &payload).unwrap();

    let _ = db
        .append_to_stream("greeting-stream", &Default::default(), greet)
        .await
        .unwrap();

    format!("Hello {}!", &name)
}

/// Produce an infinite series of `"hello"`s, one per second.
#[get("/greetings")]
async fn greetings(db_state: &State<DbState>) -> EventStream![] {
    let db = db_state.db.clone();

    let mut stream = db
        .subscribe_to_stream("greeting-stream", &Default::default())
        .await
        .unwrap();

    EventStream! {

        while let Some(event) = stream.try_next().await.unwrap() {
            if let SubEvent::EventAppeared(event) = event {

                match event.event{
                    Some( recorded_event) => {

                        match recorded_event.event_type.as_str() {
                            "greeting" => {
                                let gr_event : Greeting = recorded_event.as_json().unwrap();

                                println!("{:?}", gr_event);

                                yield Event::json(&gr_event);
                            }
                            _ =>{
                                println!("Event {} not recognized", recorded_event.event_type);
                            }
                        }

                    },
                    None => {}
                }

            }
        }
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false".parse()?;
    let client = Client::create(settings).await?;

    //
    // let result = client
    //     .read_stream("greeting-stream", &Default::default(), All)
    //     .await?;
    //
    // if let ReadResult::Ok(mut stream) = result {
    //     while let Some(event) = stream.try_next().await? {
    //         let event = event.get_original_event()
    //             .as_json::<Greeting>()?;
    //
    //         // Do something productive with the result.
    //         println!("{:?}", event);
    //     }
    // }

    rocket::build()
        .manage(DbState { db: client.clone() })
        .mount("/hello", routes![greet])
        .mount("/", routes![greetings])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await?;

    Ok(())
}
