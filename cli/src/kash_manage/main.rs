mod args;
mod config;

use self::args::{Args, Operation};
use self::config::Config;
use clap::Parser;
use kash_cli::output::OutputOptions;
use kash_repo::{fs::FsRepo, repo::RepoLike};

fn main() {
    let args: Args = Args::parse();
    let config = Config::parse().unwrap_or_default();
    let mut repo = FsRepo::new(
        &args
            .repo_dir
            .unwrap_or(config.repo.path.unwrap_or_default()),
    );

    repo.reload_store().unwrap();

    match args.op {
        Operation::Show(args) => args
            .output
            .output_format
            .to_stdout(&repo.get_all().unwrap(), OutputOptions::from(args.output)),
    };
}
