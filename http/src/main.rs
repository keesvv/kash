use kash_convert::input::{json::JsonInput, Input};
use std::io;
use std::net::Ipv4Addr;
use warp::Filter;

#[tokio::main]
async fn main() {
    let statements = JsonInput::new().from_read(io::stdin()).unwrap();

    let get_statements = warp::path!("statements").map(move || warp::reply::json(&statements));

    warp::serve(get_statements)
        .run((Ipv4Addr::LOCALHOST, 8080))
        .await;
}
