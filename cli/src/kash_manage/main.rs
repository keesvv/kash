mod args;
mod config;

use self::args::{Args, Operation};
use self::config::Config;
use clap::Parser;
use kash_cli::output::OutputOptions;
use kash_repo::{
    fs::{FsOptions, FsRepo},
    repo::RepoLike,
};
use std::fs::{self, File};
use std::io::{ErrorKind, Write};
use std::path::Path;

fn main() {
    let args: Args = Args::parse();
    let config = match Config::parse() {
        Ok(c) => c,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Ok(Default::default()),
            _ => Err(e),
        }
        .unwrap(),
    };

    let mut repo = FsRepo::new(FsOptions {
        path: args
            .repo_dir
            .unwrap_or(config.repo.unwrap_or_default().path.unwrap_or_default()),
        include: args.include,
    });

    repo.reload_store().unwrap();

    match args.op {
        Operation::Show { output } => output.output_format.to_stdout(
            &repo.get_all().unwrap(),
            OutputOptions {
                discrete: output.discrete,
                currency_symbol: output.currency_symbol.unwrap_or(
                    config
                        .output
                        .unwrap_or_default()
                        .currency
                        .unwrap_or(OutputOptions::default().currency_symbol),
                ),
            },
        ),
        Operation::New { path } => {
            let data_dir = Path::new(".kash");
            let data_dir_path = path.join(data_dir);

            fs::create_dir(&path).unwrap();
            fs::create_dir(&data_dir_path).unwrap();

            File::create(&data_dir_path.join("include")).unwrap();

            writeln!(
                File::create(path.join(".gitignore")).unwrap(),
                "{}",
                data_dir.join("cache").join("").display()
            )
            .unwrap();
        }
    };
}
