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
    let f = File::open(filename)?;
    let mut line = BufReader::new(&f).lines().next().unwrap().unwrap();

    let mut answer = 100000000;

    for problem_letter in String::from("abcdefghijklmnopqrstuvwxyz").chars() {
        let mut copied_line = line.clone();
        copied_line.retain(|c| {
            c != problem_letter.to_lowercase().to_string().chars().next().unwrap()
                && c != problem_letter.to_uppercase().to_string().chars().next().unwrap()
        });

        let cur_len = loop {
            let tmp = check_reaction(copied_line.clone()).unwrap();

            if copied_line.len() != tmp.len() {
                copied_line = check_reaction(copied_line.clone()).unwrap();
            } else {
                break copied_line.len();
            }
        };

        if cur_len < answer {
            answer = cur_len;
        }

        //println!("{}", answer.len());
    }

    Ok(answer.to_string())
}
