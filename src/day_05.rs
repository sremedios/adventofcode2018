use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn check_reaction(input: String) -> Result<String, io::Error> {
    let mut output = input.clone();

    for (i, (a, b)) in input.chars().zip(input.chars().skip(1)).enumerate() {
        if a.to_lowercase().to_string() == b.to_lowercase().to_string()
            && ((a.is_lowercase() && b.is_uppercase()) || (a.is_uppercase() && b.is_lowercase()))
        {
            output.replace_range(i..i + 2, "++");
            break;
        }
    }
    output.retain(|c| c != '+');
    Ok(output)
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let mut line = BufReader::new(&f).lines().next().unwrap().unwrap();

    let answer = loop {
        let tmp = check_reaction(line.clone()).unwrap();

        if line.len() != tmp.len() {
            line = check_reaction(line.clone()).unwrap();
        } else {
            break line;
        }
    };

    println!("{}", answer.len());

    Ok(answer.len().to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let answer = "not yet implemented";

    Ok(answer.to_string())
}
