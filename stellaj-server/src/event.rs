use crate::EventDb;
use eventstore::{ SubEvent};
use rocket::futures::TryStreamExt;
use rocket::response::stream::{Event, EventStream};
// use rocket::serde::{Deserialize, Serialize};
use public_event::{HomepageEvent, HomepageSse};
use rocket::State;


/// Produce an infinite series of `"hello"`s, one per second.
#[get("/greetings")]
pub async fn greetings(db_state: &State<EventDb>) -> EventStream![] {
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
                                let gr_event : HomepageEvent = recorded_event.as_json().unwrap();

                                println!("{:?}", gr_event);
                                let sse = HomepageSse::Event(gr_event);

                                yield Event::json(&sse);
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
