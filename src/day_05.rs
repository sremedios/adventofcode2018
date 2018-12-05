use std::fs;
use std::io;

fn solve_react(input: &str) -> Result<String, io::Error> {
    // stack implementation
    let mut output: Vec<char> = Vec::with_capacity(input.chars().count());

    for c in input.chars() {
        if output.is_empty() {
            output.push(c);
        } else if output.last().unwrap().to_lowercase().to_string() == c.to_lowercase().to_string()
            && ((output.last().unwrap().is_lowercase() && c.is_uppercase())
                || (output.last().unwrap().is_uppercase() && c.is_lowercase()))
        {
            output.pop();
        } else {
            output.push(c);
        }
    }

    Ok(output.iter().collect::<String>())
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let line = fs::read_to_string(filename)?.trim().to_string();

    let answer = solve_react(&line)?.len();

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let line = fs::read_to_string(filename)?.trim().to_string();

    let mut answer = 100_000_000;
    let alphabet_lower = String::from("abcdefghijklmnopqrstuvwxyz");
    let alphabet_upper = String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ");

    for (omit_lower, omit_upper) in alphabet_lower.chars().zip(alphabet_upper.chars()) {
        let mut copied_line = line.clone();
        copied_line.retain(|c| c != omit_lower && c != omit_upper);

        let cur_len = solve_react(&copied_line)?.len();

        if cur_len < answer {
            answer = cur_len;
        }
    }

    Ok(answer.to_string())
}
