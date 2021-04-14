use anyhow::{bail, ensure, Context, Result};
use clap::{App, Arg};
use std::fs::File;
use std::io::{Read, BufRead};
use std::path::PathBuf;


fn main() {
    let matches = App::new("grrs")
        .version("1.0.0")
        .author("nakamo326")
        .about("mini grep clone")
        .arg(Arg::with_name("patterns")
            .help("pattern to search")
            .value_name("PATTERNS")
            .index(1)
            .required(true))
        .arg(Arg::with_name("file")
            .help("file to read")
            .value_name("FILE")
            .index(2)
            .required(false))
        .get_matches();

    let pattern: String = matches.values_of("patterns").unwrap().collect();
    println!("pattern: {:?}", pattern);


}
