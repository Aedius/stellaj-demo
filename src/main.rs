mod auth;
mod event;

#[macro_use]
extern crate rocket;

use eventstore::Client;
use rocket::fs::{relative, FileServer};

pub struct EventDb {
    pub db: Client,
}

impl EventDb {
    pub fn new(db: Client) -> EventDb {
        EventDb { db }
    }
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false".parse()?;
    let event_db = Client::create(settings).await?;

    rocket::build()
        .manage(EventDb::new(event_db.clone()))
        .mount("/event", routes![event::greetings, event::greet])
        .mount("/auth", auth::get_route())
        .mount("/", FileServer::from(relative!("web")))
        .launch()
        .await?;

    Ok(())
}
