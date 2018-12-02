use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};

pub fn part_1(filename: &str) -> Result<i32, io::Error> {
    let f = File::open(filename)?;
    let answer = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|l| {
            l?.parse::<i32>().map_err(|err| {
                io::Error::new(ErrorKind::Other, format!("An error occurred: {}", err))
            })
        }).flatten()
        .sum::<i32>();
    Ok(answer)
}

pub fn part_2(filename: &str) -> Result<i32, io::Error> {
    let f = File::open(filename)?;

    let mut frequencies = HashSet::new();
    let mut frequency: i32 = 0;

    let mut reader = BufReader::new(&f);

    let answer = loop {
        frequency += match reader.by_ref().lines().next().map(|x| {
            x?.parse::<i32>().map_err(|err| {
                io::Error::new(ErrorKind::Other, format!("An error occurred: {}", err))
            })
        }) {
            Some(n) => n?,
            None => {
                reader.seek(SeekFrom::Start(0))?;
                match reader.by_ref().lines().next().map(|x| {
                    x?.parse::<i32>().map_err(|err| {
                        io::Error::new(ErrorKind::Other, format!("An error occurred: {}", err))
                    })
                }) {
                    Some(n) => n?,
                    None => 0i32,
                }
            }
        };

        if frequencies.contains(&frequency) {
            break frequency;
        } else {
            frequencies.insert(frequency);
        }
    };

    Ok(answer)
}

/*
fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let f = File::open(filename)?;

    let mut reader = BufReader::new(&f);

    // Part One
    println!(
        "Part one solution: {}",
        reader
            .by_ref()
            .lines()
            .map(|l| l?.parse::<i32>().map_err(|err| io::Error::new(
                ErrorKind::Other,
                format!("An error occurred: {}", err)
            ))).flatten()
            .sum::<i32>()
    );

    // Part Two
    reader.seek(SeekFrom::Start(0))?;

    let mut frequencies = HashSet::new();
    let mut frequency: i32 = 0;

    let answer = loop {
        frequency += match reader.by_ref().lines().next().map(|x| {
            x?.parse::<i32>().map_err(|err| {
                io::Error::new(ErrorKind::Other, format!("An error occurred: {}", err))
            })
        }) {
            Some(n) => n?,
            None => {
                reader.seek(SeekFrom::Start(0))?;
                match reader.by_ref().lines().next().map(|x| {
                    x?.parse::<i32>().map_err(|err| {
                        io::Error::new(ErrorKind::Other, format!("An error occurred: {}", err))
                    })
                }) {
                    Some(n) => n?,
                    None => 0i32,
                }
            }
        };

        if frequencies.contains(&frequency) {
            break frequency;
        } else {
            frequencies.insert(frequency);
        }
    };

    println!("Part two solution: {}", answer);

    Ok(())
}
*/
