use std::env;
use std::io;

mod day_01;
mod day_02;

fn main() -> Result<(), io::Error> {
    let args: Vec<String> = env::args().collect();

    let (part_1_answer, part_2_answer) = match args[1].as_ref() {
        "1" => (day_01::part_1(&args[2])?, day_01::part_2(&args[2])?),
        "2" => (day_02::part_1(&args[2])?, day_02::part_2(&args[2])?),
        _ => (0, 0),
    };

    println!(
        "Day {:?}\nPart 1: {:?}\nPart 2: {:?}",
        args[1], part_1_answer, part_2_answer
    );

    Ok(())
}
