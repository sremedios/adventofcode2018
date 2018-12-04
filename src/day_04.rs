use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

#[derive(Clone, Debug)]
enum State {
    BeginsShift,
    FallsAsleep,
    WakesUp,
    Undetermined,
}

#[derive(Clone, Debug)]
struct Event {
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    id: i32,
    state: State,
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        (self.month, self.day, self.hour, self.minute).cmp(&(
            other.month,
            other.day,
            other.hour,
            other.minute,
        ))
    }
}

impl Eq for Event {}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some((self.month, self.day, self.hour, self.minute).cmp(&(
            other.month,
            other.day,
            other.hour,
            other.minute,
        )))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Event) -> bool {
        self.month == other.month
            && self.day == other.day
            && self.hour == other.minute
            && self.minute == other.minute
    }
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let mut events = BufReader::new(&f)
        .by_ref()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line
                .replace("]", ",")
                .split(',')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let description = line[1].clone();

            let id = match description
                .split(' ')
                .map(|x| x.replace("#", "").to_owned())
                .into_iter()
                .filter(|x| x.parse::<i32>().is_ok())
                .next()
            {
                Some(v) => v.parse::<i32>().unwrap(),
                None => -1i32,
            };

            let state = if description.contains("falls") {
                State::FallsAsleep
            } else if description.contains("wakes") {
                State::WakesUp
            } else if description.contains("begins") {
                State::BeginsShift
            } else {
                State::Undetermined
            };

            let mut line = line[0]
                .replace("[", " ")
                .replace("-", " ")
                .replace(":", " ")
                .split(' ')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            line.retain(|entry| entry != "");
            Event {
                month: line[1].parse::<i32>().unwrap(),
                day: line[2].parse::<i32>().unwrap(),
                hour: line[3].parse::<i32>().unwrap(),
                minute: line[4].parse::<i32>().unwrap(),
                id: id,
                state: state,
            }
        }).collect::<Vec<Event>>();

    events.sort_unstable_by(|left, right| left.cmp(right));

    let mut last_id = events[0].id;
    for event in events.iter_mut().skip(1){
        if event.id == -1i32 {
            event.id = last_id;
        } else {
            last_id = event.id;
        }
    };

    // aggregate all sleeps as ID: boolean vector of minutes
    // can use .and_modify(||) .or_insert_with(||)
    let mut event_map = HashMap::new();
    for event in events {
        let mut vec_entry = event_map.entry(event.id).or_insert(Vec::new());
        vec_entry.push(event.clone());
    }
    
    // find difference between waking and sleeping
    let mut sleep_amount = HashMap::new();
    let mut diff = 0;
    let mut start_time = 0;
    let mut end_time = 0;

    for (event_id, event) in event_map.iter() {
        if event.state == State::FallsAsleep {
            start_time = event.minute;
        } else if event.state == State::WakesUp {
            end_time = event.minute; 
        }

        sleep
        
    }


    println!("{:?}", event_map.get(&1093));

    // find longest sleep
    let mut longest_sleep = 0i32;
    let mut last_minute = 0i32;
    let mut id = 0i32;
    for (event_id, event) in event_map.iter_mut() {
       if event.len() > longest_sleep as usize {
            longest_sleep = event.len() as i32;
            let final_event = event.pop().unwrap();
            last_minute = final_event.minute;
            id = final_event.id;
       }
    }

    println!("ID {:?}\nLast minute {:?}", id, last_minute);

    let answer = last_minute * id;
    Ok(answer.to_string())
}
