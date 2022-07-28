use warp::Filter;

#[tokio::main]
async fn main() {
    let root = warp::path!().map(|| "kash_http");

    warp::serve(root).run(([127, 0, 0, 1], 8080)).await;
}
