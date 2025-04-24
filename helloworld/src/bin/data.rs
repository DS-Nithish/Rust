#[allow(unused_imports)]
use clap::Parser;
#[allow(unused_imports)]
use std::{
    env,
    error::Error,
    fs::{self, File},
    io::{self, BufRead, BufReader, Read},
    path::{Path, PathBuf},
};
#[derive(Parser)]
struct Cli {
    path: PathBuf,
}
fn find_lines(path: &PathBuf) -> Result<usize, Box<dyn Error>> {
    let file = File::open(path)?;

    let buf_reader = BufReader::new(file);
    Ok(buf_reader.lines().count())
}
fn main() -> Result<(), Box<dyn Error>> {
    // let path = env::args().nth(1).expect("Error in path");
    // let args = Cli {
    //     path: PathBuf::from(path),
    // };
    let args = Cli::parse();
    let line_count = find_lines(&args.path)?;
    println!("lines count:{}", line_count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn line_check() {
        let path = PathBuf::from("ff.txt");
        let count = find_lines(&path).unwrap();
        assert_eq!(count, 17);
    }
}
