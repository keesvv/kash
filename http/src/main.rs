use kash_repo::{fs::FsRepo, repo::RepoLike};
use std::env;
use std::net::Ipv4Addr;
use std::path::PathBuf;
use warp::{reply, Filter};

#[tokio::main]
async fn main() {
    let mut repo = FsRepo::new(&PathBuf::from(env::args().nth(1).unwrap_or_default()));
    repo.reload_store().unwrap();

    let get_statements =
        warp::path!("statements").map(move || reply::json(&repo.get_all().unwrap()));

    warp::serve(get_statements)
        .run((Ipv4Addr::LOCALHOST, 8080))
        .await;
}
