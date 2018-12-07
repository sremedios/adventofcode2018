use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn traverse(
    tree: &mut HashMap<char, HashSet<char>>,
    root: char,
    children: &mut Vec<char>,
    answer: &mut String,
) -> Vec<char> {

    for c in tree[&root].clone() {
        children.push(c);
    }

    children.sort();
    tree.remove(&root);
    children.clone()
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let steps: Vec<(char, char)> = BufReader::new(&f)
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

    // tracker
    let mut tree: HashMap<char, HashSet<char>> = HashMap::new();
    let mut root: char = steps[0].0;
    let mut cur_child = steps[0].1;
    for step in steps {
        let cur_parent = step.0;
        cur_child = step.1;

        tree.entry(cur_parent)
            .and_modify(|set| {
                set.insert(cur_child);
            })
            .or_insert_with(|| {
                let mut set = HashSet::new();
                set.insert(cur_child);
                set
            });
    }

    let mut answer = String::new();
    let mut children: Vec<char> = Vec::new();
    println!("{:?}", tree);
    let mut children = traverse(&mut tree, root, &mut children, &mut answer);


    while !tree.is_empty() {
        for child in children.clone() {
            if tree.contains_key(&child) {
                let mut children = traverse(&mut tree, child, &mut children, &mut answer);
            }
        }
        match answer.find(|x| x == children[0]) {
            None => {
                answer.push(children[0]);
            }
            Some(v) => {}
        }
    }
    answer.push(cur_child);

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let answer = "unimplemented";

    Ok(answer.to_string())
}
