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


    for (i, event) in events.iter().enumerate() {
        println!("{:?}", event);
        if i > 20 {
            break;
        }

    }

    let mut last_id = events[0].id;
    for event in events.iter_mut().skip(1) {
        if event.id == -1i32 {
            event.id = last_id;
        } else {
            last_id = event.id;
        }
    }


    let mut event_map: HashMap<i32, (i32, [i32; 60])> = HashMap::new();
    let mut cur_id = -1;
    let mut asleep_minute = 0;
    let mut asleep_hour = 0;
    let mut longest_sleeper_id = -1;
    let mut longest_sleep = 0;
    let mut total_time_asleep = 0;

    for event in events {
        match event.state {
            State::BeginsShift => {
                cur_id = event.id;
            }
            State::FallsAsleep => {
                asleep_minute = event.minute;
                asleep_hour = event.hour;
            }
            State::WakesUp => {
                event_map
                    .entry(cur_id)
                    .and_modify(|(time_asleep, midnight_minutes)| {
                        //*time_asleep += event.minute - asleep_minute;
                        if asleep_hour != 0 && event.hour == 0 {
                            *time_asleep = (60i32 + event.hour - asleep_hour) % 60; 
                        } else {
                            *time_asleep = event.minute - asleep_minute;
                        }
                        total_time_asleep = *time_asleep;
                        for i in asleep_minute..event.minute {
                            midnight_minutes[i as usize] += 1;
                        }
                        if cur_id == 3491 {
                            println!("{:?}", event);
                            println!("{:?}", midnight_minutes.to_vec());
                        }
                    }).or_insert_with(|| {
                        let mut midnight_minutes = [0; 60];
                        let mut time_asleep;
                        if asleep_hour != 0 && event.hour == 0 {
                            time_asleep = (60i32 + event.hour - asleep_hour) % 60; 
                        } else {
                            time_asleep = event.minute - asleep_minute;
                        }
                        total_time_asleep = time_asleep;
                        for i in asleep_minute..event.minute {
                            midnight_minutes[i as usize] += 1;
                        }
                        (time_asleep, midnight_minutes)
                    });
                if total_time_asleep > longest_sleep {
                    //println!("{:?}", event);
                    longest_sleep = total_time_asleep;
                    longest_sleeper_id = event.id;
                }
            }
            State::Undetermined => {}
        }
    }

    // argmax
    let mut most_freq_minute_asleep: i32 = 0;
    let mut max = 0;
    for (i, minute) in event_map
        .get(&longest_sleeper_id)
        .unwrap()
        .1
        .iter()
        .enumerate()
        .skip(1)
    {
        if max < *minute {
            most_freq_minute_asleep = i as i32;
            max = *minute;
        }
    }

    event_map
        .get(&longest_sleeper_id)
        .unwrap()
        .1
        .iter()
        .max()
        .unwrap();

    /*
    println!(
        "{:?}",
        event_map.get(&longest_sleeper_id).unwrap().1.to_vec()
    );
    */

    println!(
        "ID: {:?}\nLast sleeping minute: {:?}",
        longest_sleeper_id, most_freq_minute_asleep
    );

    let answer = longest_sleeper_id * most_freq_minute_asleep;
    Ok(answer.to_string())
}
