use std::env;
use std::io;
use std::path::PathBuf;

mod day_01;
mod day_02;

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

    let input_filename = input_filename.to_str().unwrap();

    println!("{}", input_filename);

    let (part_1_answer, part_2_answer) = match day.as_ref() {
        "01" => (
            day_01::part_1(input_filename)?,
            day_01::part_2(input_filename)?,
        ),
        "02" => (
            day_02::part_1(input_filename)?,
            day_02::part_2(input_filename)?,
        ),
        _ => ("Invalid argument".to_owned(), "Invalid argument".to_owned()),
    };

    println!(
        "Day {:?}\nPart 1: {:?}\nPart 2: {:?}",
        args[1], part_1_answer, part_2_answer
    );

    Ok(())
}
