use chrono::prelude::*;
use chrono::Utc;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Clone, Debug)]
enum State {
    BeginsShift,
    FallsAsleep,
    WakesUp,
    Undetermined,
}

#[derive(Clone, Debug)]
struct Event {
    time: DateTime<Utc>,
    id: i32,
    state: State,
}

pub fn part_1(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let mut events = BufReader::new(&f)
        .lines()
        .map(|line| {
            let line = line
                .unwrap()
                .parse::<String>()
                .unwrap()
                .split("] ")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let time = Utc.datetime_from_str(&line[0], "[%Y-%m-%d %H:%M").unwrap();
            let id = match line[1]
                .split(' ')
                .map(|x| x.replace("#", "").to_owned())
                .find(|x| x.parse::<i32>().is_ok())
            {
                Some(v) => v.parse::<i32>().unwrap(),
                None => -1i32,
            };

            let state = match line[1].split(' ').next().unwrap() {
                "falls" => State::FallsAsleep,
                "wakes" => State::WakesUp,
                "Guard" => State::BeginsShift,
                _ => State::Undetermined,
            };

            let mut line = line[0]
                .replace("[", " ")
                .replace("-", " ")
                .replace(":", " ")
                .split(' ')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            line.retain(|entry| entry != "");
            Event { time, id, state }
        }).collect::<Vec<Event>>();

    events.sort_unstable_by(|left, right| left.time.cmp(&right.time));

    let mut event_map: HashMap<i32, (i32, [i32; 60])> = HashMap::new();
    let mut cur_id = -1;
    let mut sleep_start: DateTime<Utc> = Utc::now();
    let mut longest_sleeper_id = -1;
    let mut longest_sleep = 0;
    let mut total_time_asleep = 0;

    for event in events {
        match event.state {
            State::BeginsShift => {
                // only update cur_id when new guards
                cur_id = event.id;
            }
            State::FallsAsleep => {
                sleep_start = event.time;
            }
            State::WakesUp => {
                event_map
                    .entry(cur_id)
                    .and_modify(|(time_asleep, midnight_minutes)| {
                        *time_asleep += (event.time - sleep_start).num_minutes() as i32;
                        total_time_asleep = *time_asleep;
                        for i in sleep_start.minute()..event.time.minute() {
                            midnight_minutes[i as usize] += 1;
                        }
                    }).or_insert_with(|| {
                        let mut midnight_minutes = [0; 60];
                        let time_asleep = (event.time - sleep_start).num_minutes() as i32;
                        total_time_asleep = time_asleep;
                        for i in sleep_start.minute()..event.time.minute() {
                            midnight_minutes[i as usize] += 1;
                        }
                        (time_asleep, midnight_minutes)
                    });
                if total_time_asleep > longest_sleep {
                    longest_sleep = total_time_asleep;
                    longest_sleeper_id = cur_id;
                }
            }
            State::Undetermined => {}
        }
    }

    // argmax
    let mut most_freq_minute_asleep: i32 = 0;
    let mut max = 0;
    for (i, minute) in event_map[&longest_sleeper_id].1.iter().enumerate().skip(1) {
        if max < *minute {
            most_freq_minute_asleep = i as i32;
            max = *minute;
        }
    }

    event_map[&longest_sleeper_id].1.iter().max().unwrap();

    let answer = longest_sleeper_id * most_freq_minute_asleep;

    Ok(answer.to_string())
}

pub fn part_2(filename: &str) -> Result<String, io::Error> {
    let f = File::open(filename)?;
    let mut events = BufReader::new(&f)
        .lines()
        .map(|line| {
            let line = line
                .unwrap()
                .parse::<String>()
                .unwrap()
                .split("] ")
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            let time = Utc.datetime_from_str(&line[0], "[%Y-%m-%d %H:%M").unwrap();
            let id = match line[1]
                .split(' ')
                .map(|x| x.replace("#", "").to_owned())
                .find(|x| x.parse::<i32>().is_ok())
            {
                Some(v) => v.parse::<i32>().unwrap(),
                None => -1i32,
            };

            let state = match line[1].split(' ').next().unwrap() {
                "falls" => State::FallsAsleep,
                "wakes" => State::WakesUp,
                "Guard" => State::BeginsShift,
                _ => State::Undetermined,
            };

            let mut line = line[0]
                .replace("[", " ")
                .replace("-", " ")
                .replace(":", " ")
                .split(' ')
                .map(|x| x.to_owned())
                .collect::<Vec<String>>();
            line.retain(|entry| entry != "");
            Event { time, id, state }
        }).collect::<Vec<Event>>();

    events.sort_unstable_by(|left, right| left.time.cmp(&right.time));

    let mut event_map: HashMap<i32, (i32, [i32; 60])> = HashMap::new();
    let mut cur_id = -1;
    let mut sleep_start: DateTime<Utc> = Utc::now();

    for event in events {
        match event.state {
            State::BeginsShift => {
                // only update cur_id when new guards
                cur_id = event.id;
            }
            State::FallsAsleep => {
                sleep_start = event.time;
            }
            State::WakesUp => {
                event_map
                    .entry(cur_id)
                    .and_modify(|(time_asleep, midnight_minutes)| {
                        *time_asleep += (event.time - sleep_start).num_minutes() as i32;
                        for i in sleep_start.minute()..event.time.minute() {
                            midnight_minutes[i as usize] += 1;
                        }
                    }).or_insert_with(|| {
                        let mut midnight_minutes = [0; 60];
                        let time_asleep = (event.time - sleep_start).num_minutes() as i32;
                        for i in sleep_start.minute()..event.time.minute() {
                            midnight_minutes[i as usize] += 1;
                        }
                        (time_asleep, midnight_minutes)
                    });
            }
            State::Undetermined => {}
        }
    }

    // argmax
    let mut sleepiest_id: i32 = -1;
    let mut times_asleep = 0;
    let mut most_freq_minute_asleep: i32 = 0;

    for (id, midnight_minutes) in event_map {
        for (i, freq) in midnight_minutes.1.iter().enumerate() {
            if times_asleep < *freq {
                most_freq_minute_asleep = i as i32;
                times_asleep = *freq;
                sleepiest_id = id;
            }
        }
    }

    let answer = sleepiest_id * most_freq_minute_asleep;

    Ok(answer.to_string())
}
