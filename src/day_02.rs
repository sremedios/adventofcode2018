use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let reader = BufReader::new(&f);

    let mut n_dups = 0;
    let mut n_trips = 0;

    for line in reader.lines() {
        let mut occurrences = HashMap::new();
        for c in line?.chars() {
            let count = occurrences.entry(c).or_insert(0);
            *count += 1;
        }

        if occurrences.iter().any(|(_key, val)| *val == 2) {
            n_dups += 1;
        }
        if occurrences.iter().any(|(_key, val)| *val == 3) {
            n_trips += 1;
        }
    }

    let answer = n_dups * n_trips;

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;

    let reader = BufReader::new(&f);

    let box_ids: Vec<String> = reader.lines().flatten().collect();

    let mut answer = String::new();

    for word_1 in box_ids.iter() {
        for word_2 in box_ids.iter() {
            let num_mismatched: u8 = word_1
                .chars()
                .zip(word_2.chars())
                .map(
                    |(letter_1, letter_2)| {
                        if letter_1 == letter_2 {
                            0
                        } else {
                            1
                        }
                    },
                ).sum();
            if num_mismatched == 1u8 {
                answer = word_1
                    .chars()
                    .zip(word_2.chars())
                    .filter(|(letter_1, letter_2)| letter_1 == letter_2)
                    .map(|(letter_1, _letter_2)| letter_1)
                    .collect();
            }
        }
    }

    Ok(answer)
}
