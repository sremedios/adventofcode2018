use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;

    println!(
        "{}",
        BufReader::new(f)
            .lines()
            .map(|l| l?.parse::<i32>().map_err(|err| io::Error::new(
                ErrorKind::Other,
                format!("An error occurred: {}", err)
            ))).flatten()
            .sum::<i32>()
    );

    Ok(())
}
