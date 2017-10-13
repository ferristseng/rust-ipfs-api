extern crate futures;
extern crate ipfs_api;
extern crate tokio_core;
extern crate tokio_timer;

use futures::Future;
use futures::stream::Stream;
use ipfs_api::{response, IpfsClient};
use std::thread;
use std::time::Duration;
use tokio_core::reactor::{Core, Handle};
use tokio_timer::Timer;

static TOPIC: &'static str = "test";

fn get_client(handle: &Handle) -> IpfsClient {
    println!("connecting to localhost:5001...");

    IpfsClient::default(handle)
}

// Creates an Ipfs client, and simultaneously publishes and reads from a pubsub
// topic.
//
fn main() {
    // This block will execute a repeating function that sends
    // a message to the "test" topic.
    //
    thread::spawn(move || {
        let mut event_loop = Core::new().expect("expected event loop");
        let handle = event_loop.handle();
        let timer = Timer::default();
        let publish = timer
            .interval(Duration::from_secs(1))
            .map_err(|_| {
                response::Error::Uncategorized("timeout error".to_string())
            })
            .for_each(move |_| {
                println!("");
                println!("publishing message...");

                get_client(&handle)
                    .pubsub_pub(TOPIC, "Hello World!")
                    .then(|_| {
                        println!("success");
                        Ok(())
                    })
            });

        println!("");
        println!("starting task to publish messages to ({})...", TOPIC);
        event_loop.run(publish).expect(
            "expected the publish task to start",
        );
    });

    // This block will execute a future that suscribes to a topic,
    // and reads any incoming messages.
    //
    {
        let mut event_loop = Core::new().expect("expected event loop");
        let client = get_client(&event_loop.handle());
        let req = client.pubsub_sub(TOPIC, None);

        println!("");
        println!("waiting for messages on ({})...", TOPIC);
        event_loop
            .run(req.for_each(|msg| {
                println!("");
                println!("received ({:?})", msg);

                Ok(())
            }))
            .expect("expected a valid response");
    }
}
