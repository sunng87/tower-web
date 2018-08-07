extern crate bytes;
extern crate futures;
#[macro_use]
extern crate tower_web;
extern crate tokio;

use bytes::Buf;
use futures::{Future, IntoFuture};
use tower_web::ServiceBuilder;
use tower_web::util::BufStream;

use std::io;

#[derive(Clone, Debug)]
struct BigBody;

impl_web! {
    impl BigBody {
        /// @post("/upload")
        fn upload<B>(&self, body: B) -> impl Future<Item = String> + Send
        where B: BufStream + Send,
              B::Item: Send,
              B::Error: Send,
        {
            body.for_each(|chunk| {
                println!("CHUNK LEN = {}", chunk.remaining());
                Ok(())
            })
            .and_then(|_| {
                Ok("all done".to_string())
            })
        }
    }
}

pub fn main() {
    let addr = "127.0.0.1:8080".parse().expect("Invalid address");
    println!("Listening on http://{}", addr);

    ServiceBuilder::new()
        .resource(BigBody)
        .run(&addr)
        .unwrap();
}
