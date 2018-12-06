use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn manhattan_dist(x1: u32, y1: u32, x2: u32, y2: u32) -> i32 {
    (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs() as i32
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let coords: Vec<(u32, u32)> = BufReader::new(&f)
        .lines()
        .map(|line| {
            let line = line
                .unwrap()
                .split(", ")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            (
                line[0].parse::<u32>().unwrap(),
                line[1].parse::<u32>().unwrap(),
            )
        }).collect();

    // get maximum grid size
    let mut max = 0;
    for coord in coords.clone() {
        if max < coord.0 {
            max = coord.0;
        }
        if max < coord.1 {
            max = coord.1;
        }
    }
    max += 10; // for safety for now

    let mut coord_map: HashMap<(u32, u32), i32> = HashMap::new();
    let mut closest_dist = 100_000;
    let mut closest_coord = (0u32, 0u32);

    // find to which point each grid location is closest
    for i in 0..max {
        for j in 0..max {
            closest_dist = 100_000;
            closest_coord = (0u32, 0u32);

            // calculate all distances to remove duplicates
            let mut dists: Vec<i32> = Vec::new();
            for coord in coords.clone() {
                dists.push(manhattan_dist(i, j, coord.0, coord.1));
            }

            if dists
                .clone()
                .iter()
                .filter(|&n| *n == *dists.iter().min().unwrap())
                .count()
                > 1
            {
                continue;
            }

            // if any duplicates occur, skip this iteration

            for coord in coords.clone() {
                let cur_dist = manhattan_dist(i, j, coord.0, coord.1);
                if cur_dist < closest_dist {
                    closest_dist = cur_dist;
                    closest_coord = coord;
                }
            }
            // increment area since this point was closest
            coord_map
                .entry(closest_coord)
                .and_modify(|x| *x += 1)
                .or_insert(1);

            // prune out infinite areas
            if i == 0 || j == 0 || i == max - 1 || j == max - 1 {
                coord_map
                    .entry(closest_coord)
                    .and_modify(|x| *x -= 100_000)
                    .or_insert(-100_000);
            }
        }
    }

    coord_map.retain(|_, v| *v > 0);

    let answer = coord_map.values().max().unwrap();

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let answer = "unimplemented";

    Ok(answer.to_string())
}
