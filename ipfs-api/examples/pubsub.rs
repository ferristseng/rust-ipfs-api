// Copyright 2017 rust-ipfs-api Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.
//

#[macro_use]
extern crate futures;

use futures::{future, FutureExt, StreamExt, TryStreamExt};
use ipfs_api::IpfsClient;
use std::time::Duration;
use tokio::time;
use tokio_stream::wrappers::IntervalStream;

static TOPIC: &'static str = "test";

fn get_client() -> IpfsClient {
    eprintln!("connecting to localhost:5001...");

    IpfsClient::default()
}

// Creates an Ipfs client, and simultaneously publishes and reads from a pubsub
// topic.
//
#[cfg_attr(feature = "with-actix", actix_rt::main)]
#[cfg_attr(feature = "with-hyper", tokio::main)]
async fn main() {
    eprintln!("note: ipfs must be run with the --enable-pubsub-experiment flag");

    let publish_client = get_client();

    // This block will execute a repeating function that sends
    // a message to the "test" topic.
    //
    let interval = time::interval(Duration::from_secs(1));
    let mut publish = IntervalStream::new(interval)
        .then(|_| future::ok(())) // Coerce the stream into a TryStream
        .try_for_each(|_| {
            eprintln!();
            eprintln!("publishing message...");

            publish_client
                .pubsub_pub(TOPIC, "Hello World!")
                .boxed_local()
        })
        .boxed_local()
        .fuse();

    // This block will execute a future that suscribes to a topic,
    // and reads any incoming messages.
    //
    let mut subscribe = {
        let client = get_client();

        client
            .pubsub_sub(TOPIC, false)
            .take(5)
            .try_for_each(|msg| {
                eprintln!();
                eprintln!("received ({:?})", msg);

                future::ok(())
            })
            .fuse()
    };

    eprintln!();
    eprintln!("publish messages to ({})...", TOPIC);
    eprintln!("waiting for messages from ({})...", TOPIC);

    select! {
        res = publish => if let Err(e) = res {
            eprintln!("error publishing messages: {}", e);
        },
        res = subscribe => match res {
            Ok(_) => eprintln!("done reading messages..."),
            Err(e) => eprintln!("error reading messages: {}", e)
        },
    }
}
