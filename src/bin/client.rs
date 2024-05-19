use futures::{Stream, TryStreamExt};
use std::{ time::Duration};

use eventsource_client as es;

#[tokio::main]
async fn main() -> Result<(), es::Error> {
    let url = "http://localhost:3000/api/events";

    let client = es::ClientBuilder::for_url(url)?
        .reconnect(
            es::ReconnectOptions::reconnect(true)
                .retry_initial(false)
                .delay(Duration::from_secs(1))
                .backoff_factor(2)
                .delay_max(Duration::from_secs(60))
                .build(),
        )
        .build();

    let mut stream = tail_events(client);

    while let Ok(Some(_)) = stream.try_next().await {}

    Ok(())
}

fn tail_events(client: impl es::Client) -> impl Stream<Item = Result<(), ()>> {
    client
        .stream()
        .map_ok(|event| match event {
            es::SSE::Event(ev) => {
                println!("got an event: {}\n{}", ev.event_type, ev.data)
            }
            es::SSE::Comment(comment) => {
                println!("got a comment: \n{}", comment)
            }
        })
        .map_err(|err| eprintln!("error streaming events: {:?}", err))
}