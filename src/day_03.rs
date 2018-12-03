use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind, Read, Seek, SeekFrom};

#[derive(Clone, Debug)]
struct Fabric {
    start_x: u32,
    start_y: u32,
    width: u32,
    height: u32,
    id: u32,
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let exclude_symbols = vec!['#', '@', ',', 'x', ':'];
    let fabrics = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut line = line
                .replace("#", " ")
                .replace("@", " ")
                .replace(",", " ")
                .replace("x", " ")
                .replace(":", " ")
                .split(' ')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            line.retain(|entry| entry != "");
            Fabric {
                start_x: line[1].parse::<u32>().unwrap(),
                start_y: line[2].parse::<u32>().unwrap(),
                width: line[3].parse::<u32>().unwrap(),
                height: line[4].parse::<u32>().unwrap(),
                id: line[0].parse::<u32>().unwrap(),
            }
        }).collect::<Vec<Fabric>>();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; 2000]; 2000];

    for fabric in fabrics {
        for w in 0..fabric.width {
            for h in 0..fabric.height {
                grid[(fabric.start_x + w) as usize][(fabric.start_y + h) as usize] += 1;
            }
        }
    }
    let mut sum: u32 = grid
        .iter()
        .flat_map(|v| v.iter().filter(|x| *x >= &2).map(|x| 1))
        .sum();

    let answer = format!("{}", sum);
    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let exclude_symbols = vec!['#', '@', ',', 'x', ':'];
    let fabrics = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut line = line
                .replace("#", " ")
                .replace("@", " ")
                .replace(",", " ")
                .replace("x", " ")
                .replace(":", " ")
                .split(' ')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            line.retain(|entry| entry != "");
            Fabric {
                start_x: line[1].parse::<u32>().unwrap(),
                start_y: line[2].parse::<u32>().unwrap(),
                width: line[3].parse::<u32>().unwrap(),
                height: line[4].parse::<u32>().unwrap(),
                id: line[0].parse::<u32>().unwrap(),
            }
        }).collect::<Vec<Fabric>>();

    let mut grid: Vec<Vec<u32>> = vec![vec![0; 2000]; 2000];

    let mut all_ids: Vec<u32> = fabrics.iter().map(|f| f.id).collect();
    let mut wrong_ids: Vec<u32> = Vec::new();

    let mut answer = 0u32;
    for fabric in fabrics.clone() {
        for w in 0..fabric.width {
            for h in 0..fabric.height {
                grid[(fabric.start_x + w) as usize][(fabric.start_y + h) as usize] += 1;
            }
        }
    }

    for fabric in fabrics {
        for w in 0..fabric.width {
            for h in 0..fabric.height {
                if grid[(fabric.start_x + w) as usize][(fabric.start_y + h) as usize] > 1 {
                    wrong_ids.push(fabric.id);
                }
            }
        }
    }

    all_ids.retain(|id| !wrong_ids.contains(id));
    answer = all_ids[0];

    Ok(answer.to_string())
}
