mod event;

#[macro_use]
extern crate rocket;

use event::DbState;
use eventstore::{Client};
use rocket::fs::{relative, FileServer};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false".parse()?;
    let client = Client::create(settings).await?;

    rocket::build()
        .manage(DbState::new(client.clone()))
        .mount("/hello", routes![event::greet])
        .mount("/", routes![event::greetings])
        .mount("/", FileServer::from(relative!("static")))
        .launch()
        .await?;

    Ok(())
}
