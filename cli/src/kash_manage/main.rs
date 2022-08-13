mod args;

use self::args::{Args, Operation};
use clap::Parser;
use kash_cli::output::OutputOptions;
use kash_manage::{fs::FsRepo, repo::RepoLike};

fn main() {
    let args: Args = Args::parse();
    let mut repo = FsRepo::new(&args.repo_dir);

    repo.reload_store().unwrap();

    match args.op {
        Operation::Show(args) => args.output.output_format.to_stdout(
            &repo.get_all().unwrap(),
            OutputOptions {
                discrete: args.output.discrete,
            },
        ),
    };
}
