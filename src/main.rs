extern crate exitcode;

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

fn main() {
    let args = Cli::parse();
    let result: Result<bool>;
    if let Some(path) = args.path {
        let f = File::open(path);
        match f {
            Ok(x) => {
                let reader = BufReader::new(x);
                result = search(reader, args.pattern);
            },
            Err(e) => {
                eprintln!("grrs: {}", e);
                std::process::exit(2)
            },
        };
    } else {
        let stdin = stdin();
        let reader = stdin.lock();
        result = search(reader, args.pattern);
    }
    match result {
        Ok(true) => std::process::exit(exitcode::OK),
        Ok(false) => std::process::exit(1),
        Err(e) => {
            eprintln!("grrs: {:?}", e);
            std::process::exit(2)
        },
    };
}

fn search<R: BufRead>(reader: R, pattern: String) -> Result<bool> {
    let mut has_match = false;
    for line in reader.lines() {
        let line = line?;
        if line.contains(&pattern) {
            has_match = true;
            println!("{}", line);
        }
    }
    Ok(has_match)
}
