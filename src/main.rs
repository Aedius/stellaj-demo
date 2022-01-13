use eventstore::{ All, Client, EventData, ReadResult };
use futures::stream::TryStreamExt;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Foo {
    is_rust_a_nice_language: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Creates a client settings for a single node configuration.
    let settings = "esdb://admin:changeit@localhost:2113?tls=false&tlsVerifyCert=false".parse()?;
    let client = Client::create(settings).await?;

    let payload = Foo {
        is_rust_a_nice_language: true,
    };

    // It is not mandatory to use JSON as a data format however EventStoreDB
    // provides great additional value if you do so.
    let evt = EventData::json("language-poll", &payload)?;

    let _ = client
        .append_to_stream("language-stream", &Default::default(), evt)
        .await?;

    let result = client
        .read_stream("language-stream", &Default::default(), All)
        .await?;

    if let ReadResult::Ok(mut stream) = result {
        while let Some(event) = stream.try_next().await? {
            let event = event.get_original_event()
                .as_json::<Foo>()?;

            // Do something productive with the result.
            println!("{:?}", event);
        }
    }

    Ok(())
}