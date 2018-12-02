use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};

pub fn part_1(filename: &str) -> Result<i32, io::Error> {
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

        let mut allow_dup = true;
        let mut allow_trip = true;

        for (_key, val) in occurrences.iter() {
            if *val == 2 && allow_dup {
                n_dups += 1;
                allow_dup = false;
            }
            if *val == 3 && allow_trip {
                n_trips += 1;
                allow_trip = false;
            }
        }
    }

    let answer = n_dups * n_trips;

    Ok(answer)
}

pub fn part_2(filename: &str) -> Result<i32, io::Error> {
    let f = File::open(filename)?;

    let reader = BufReader::new(&f);

    let box_ids: Vec<String> = reader.lines().flatten().collect();


    for word_1 in box_ids.iter() {
        for word_2 in box_ids.iter() {
            let mut wrong_count = 0;

            for (letter_1, letter_2) in word_1.chars().zip(word_2.chars()) {

                if letter_1 != letter_2 {
                    wrong_count += 1;
                } 
            }
            if wrong_count == 1 {
                println!("{}\n{}", word_1, word_2);
            }
        }
    }

    /*

    box_ids.iter()
        .zip(box_ids.iter().cycle().skip(1))
        .map(move |(word_1, word_2)| {
            let mut correct_count = 0;
            word_1.chars()
                .zip(word_2.chars())
                .map(move |(letter_1, letter_2)| {
                    println!("Correct count: {}", correct_count);
                    if letter_1 == letter_2 {
                        correct_count += 1;
                    } else if correct_count == 4 {
                        println!("{}\n{}", word_1, word_2);
                    }
                })
        });
    */

    Ok(1i32)
}
