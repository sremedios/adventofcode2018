extern crate chrono;

use std::env;
use std::io;
use std::path::PathBuf;

mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();
    let day = if args[1].chars().count() == 1 {
        format!("0{}", args[1])
    } else {
        args[1].to_string()
    };
    let input_filename: PathBuf = [
        "resources",
        format!("{}{}{}", "day_", day, "_input.txt").as_ref(),
    ]
    .iter()
    .collect();

    let input_filename = match input_filename.to_str() {
        Some(f) => f,
        None => "error_parsing_filename",
    };

    let (part_1_answer, part_2_answer) = match day.as_ref() {
        "01" => (
            day_01::part_1(input_filename)?,
            day_01::part_2(input_filename)?,
        ),
        "02" => (
            day_02::part_1(input_filename)?,
            day_02::part_2(input_filename)?,
        ),
        "03" => (
            day_03::part_1(input_filename)?,
            day_03::part_2(input_filename)?,
        ),
        "04" => (
            day_04::part_1(input_filename)?,
            day_04::part_2(input_filename)?,
        ),

        "05" => (
            day_05::part_1(input_filename)?,
            day_05::part_2(input_filename)?,
        ),
        "06" => (
            day_06::part_1(input_filename)?,
            day_06::part_2(input_filename)?,
        ),
        "07" => (
            day_07::part_1(input_filename)?,
            day_07::part_2(input_filename)?,
        ),
        _ => ("Invalid argument".to_owned(), "Invalid argument".to_owned()),
    };

    println!(
        "===== Day {} =====\nPart 1: {}\nPart 2: {}",
        args[1], part_1_answer, part_2_answer
    );

    Ok(())
}
