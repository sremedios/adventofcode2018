use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead, BufReader};

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let mut stack = fs::read_to_string(filename)?
        .split(' ')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    println!("{:?}", stack);

    // read backwards to make use of stack functionality
    stack.reverse();

    let mut num_children = stack.pop().unwrap();
    let mut num_metadata = stack.pop().unwrap();

    let mut metadata: Vec<u32> = Vec::new();

    while !stack.is_empty() {
        for item in 0..(num_children + num_metadata) {
            println!("Child: {:?}", item);
            println!("Num children: {:?}", num_children);
            println!("Num metadata: {:?}", num_metadata);
            num_children = match stack.pop() {
                Some(v) => v,
                None => 0,
            };
            num_metadata = match stack.pop() {
                Some(v) => v,
                None => 0,
            };
            for item in 0..num_metadata {
                match stack.pop() {
                    Some(v) => {
                        println!("Pushing: {:?}", v);
                        metadata.push(v);
                    }
                    None => (),
                }
            }
        }
    }

    println!("{:?}", metadata);

    let answer = "unfinished";

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let answer = "Unimplemented";

    Ok(answer.to_string())
}
