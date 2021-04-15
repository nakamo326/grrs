use anyhow::{bail, ensure, Context, Result};
use clap::Clap;
use std::fs::File;
use std::io::{stdin, BufRead, BufReader};
use std::path::PathBuf;


#[derive(Clap, Debug)]
#[clap(
    name = "grrs",
    version = "1.0.0",
    author = "nakamo326",
    about = "mini grep clone"
)]
struct Cli {
    /// pattern to search each lines
    #[clap(name = "PATTERN")]
    pattern: String,
    /// target file
    #[clap(name = "FILE")]
    path: Option<PathBuf>,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    if let Some(path) = args.path {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        search(reader, args.pattern)
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        search(reader, args.pattern)
    }
}

fn search<R: BufRead>(reader: R, pattern: String) -> Result<()> {
    for line in reader.lines() {
        let line = line?;
        if line.contains(&pattern) {
            println!("{}", line);
        }
    }
    Ok(())
}
