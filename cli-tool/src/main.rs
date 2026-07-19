#![allow(unused_imports)]
use clap::Parser;
use std::io::{prelude::*, read_to_string};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);

    for line in reader.lines() {
        match line {
            Ok(line) if line.contains(&args.pattern) => println!("{}", line),
            Ok(_) => (),
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}
