use crate::EventDb;

use rocket::State;
use eventstore::{EventData};
use public_event::{HomepageEvent, Player};
use crate::auth::KeyCloakUser;
use rocket::Route;


pub fn get_route() -> Vec<Route> {
    return routes![name];
}

#[get("/name")]
pub async fn name(db_state: &State<EventDb>, user: KeyCloakUser) -> String {
    let db = db_state.db.clone();

    let payload = HomepageEvent::NewPlayer(Player {
        pseudo: user.username.to_string(),
        is_bot: false,
    });
    let greet = EventData::json("greeting", &payload).unwrap();

    let _ = db
        .append_to_stream("greeting-stream", &Default::default(), greet)
        .await
        .unwrap();

    format!("Hello {}!", &user.username)
}