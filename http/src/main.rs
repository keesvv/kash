mod args;

use self::args::Args;
use clap::Parser;
use kash_repo::{
    fs::{FsOptions, FsRepo},
    repo::RepoLike,
};
use std::net::Ipv4Addr;
use warp::{reply, Filter};

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let mut repo = FsRepo::new(FsOptions {
        include: Vec::new(),
        path: args.repo.unwrap_or_default(),
    });

    repo.reload_store().unwrap();

    let get_statements =
        warp::path!("statements").map(move || reply::json(&repo.get_all().unwrap()));

    warp::serve(get_statements)
        .run((Ipv4Addr::LOCALHOST, args.port))
        .await;
}
