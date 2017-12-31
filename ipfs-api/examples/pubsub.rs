// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate ipfs_api;
extern crate tokio_core;
extern crate tokio_timer;

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
        let client = get_client(&event_loop.handle());
        let timer = Timer::default();
        let publish = timer
            .interval(Duration::from_secs(1))
            .map_err(|_| response::Error::from("timeout error"))
            .for_each(move |_| {
                println!();
                println!("publishing message...");

                client.pubsub_pub(TOPIC, "Hello World!")
            });

        println!();
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
        let req = client.pubsub_sub(TOPIC, false);

        println!();
        println!("waiting for messages on ({})...", TOPIC);
        event_loop
            .run(req.take(5).for_each(|msg| {
                println!();
                println!("received ({:?})", msg);

                Ok(())
            }))
            .expect("expected a valid response");
    }
}
