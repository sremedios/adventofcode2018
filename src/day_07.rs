use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let deps: Vec<(char, char)> = BufReader::new(&f)
        .lines()
        .map(|line| {
            let line = line
                .unwrap()
                .split(" ")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            (
                line[1].parse::<char>().unwrap(),
                line[7].parse::<char>().unwrap(),
            )
        })
        .collect();

    // Get a set of all steps
    let steps: Vec<char> = deps
        .iter()
        .cloned()
        .flat_map(|(requirement, step)| vec![requirement, step])
        .collect();

    // graph
    let mut tree: HashMap<char, HashSet<char>> = HashMap::new();

    for step in &steps {
        tree.entry(*step).or_insert_with(HashSet::new);
    }

    for (requirement, step) in &deps {
        tree.entry(*step).and_modify(|requirements| {
            requirements.insert(*requirement);
        });
    }

    let mut answer = String::new();

    while !tree.is_empty() {
        let mut next_states: Vec<char> = tree
            .iter()
            .filter(|(_, requirements)| (*requirements).is_empty())
            .map(|(step, _)| step)
            .cloned()
            .collect();

        next_states.sort();

        let state = next_states.first().unwrap();
        tree.remove(state);
        for c in tree.values_mut() {
            c.remove(&state);
        }

        answer.push(*state);
    }

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let answer = "unimplemented";

    Ok(answer.to_string())
}
