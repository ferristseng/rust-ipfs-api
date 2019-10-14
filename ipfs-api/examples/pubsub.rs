// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

extern crate futures;
extern crate ipfs_api;
extern crate tokio;
extern crate tokio_timer;

use futures::{Future, Stream};
use ipfs_api::IpfsClient;
use std::time::{Duration, Instant};
use tokio::runtime::current_thread::Runtime;
use tokio_timer::Interval;

static TOPIC: &'static str = "test";

fn get_client() -> IpfsClient {
    println!("connecting to localhost:5001...");

    IpfsClient::default()
}

// Creates an Ipfs client, and simultaneously publishes and reads from a pubsub
// topic.
//
fn main() {
    let mut rt = Runtime::new().expect("tokio runtime");

    // This block will execute a repeating function that sends
    // a message to the "test" topic.
    //
    {
        let client = get_client();
        let publish = Interval::new(Instant::now(), Duration::from_secs(1))
            .map_err(|e| eprintln!("{}", e))
            .for_each(move |_| {
                println!();
                println!("publishing message...");

                client
                    .pubsub_pub(TOPIC, "Hello World!")
                    .map_err(|e| eprintln!("{}", e))
            });

        println!();
        println!("starting task to publish messages to ({})...", TOPIC);

        rt.spawn(publish);
    }

    // This block will execute a future that suscribes to a topic,
    // and reads any incoming messages.
    //
    {
        let client = get_client();
        let req = client.pubsub_sub(TOPIC, false);

        println!();
        println!("waiting for messages on ({})...", TOPIC);
        let fut = req
            .take(5)
            .for_each(|msg| {
                println!();
                println!("received ({:?})", msg);

                Ok(())
            })
            .map_err(|e| eprintln!("{}", e));

        rt.block_on(fut).expect("successful response");
    }
}
