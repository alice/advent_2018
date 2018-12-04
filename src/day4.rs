use chrono::{NaiveDate, NaiveDateTime, Timelike};
use regex::Regex;
use std::cmp::{Eq, Ordering};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
enum Event {
  BeginsShift(i32),
  FallsAsleep,
  WakesUp,
}

#[derive(Debug)]
struct EventParseError {}

impl FromStr for Event {
  type Err = EventParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref event_re: Regex = Regex::new(r"(Guard #(?P<g>\d+) )?(?P<e>[\w\s]+)").unwrap();
    };
    let captures = event_re.captures(s).unwrap();
    let event = captures.name("e").unwrap().as_str();
    let guard = captures.name("g");
    match event {
      "falls asleep" => return Ok(Event::FallsAsleep),
      "wakes up" => return Ok(Event::WakesUp),
      "begins shift" => return Ok(Event::BeginsShift(guard.unwrap().as_str().parse().unwrap())),
      _ => return Err(EventParseError {}),
    }
  }
}

#[derive(Debug)]
pub struct Log {
  timestamp: NaiveDateTime,
  event: Event,
}

#[derive(Debug)]
pub struct LogParseError {}

impl FromStr for Log {
  type Err = LogParseError;

  // [1518-11-01 00:00] Guard #10 begins shift
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref log_re: Regex =
        Regex::new(r"\[(?P<y>\d+)-(?P<m>\d+)-(?P<d>\d+) (?P<hh>\d+):(?P<mm>\d+)\] (?P<e>.+)")
          .unwrap();
    };
    let captures = log_re.captures(s).unwrap();

    let year: i32 = captures.name("y").unwrap().as_str().parse().unwrap();
    let month: u32 = captures.name("m").unwrap().as_str().parse().unwrap();
    let day: u32 = captures.name("d").unwrap().as_str().parse().unwrap();
    let hour: u32 = captures.name("hh").unwrap().as_str().parse().unwrap();
    let minute: u32 = captures.name("mm").unwrap().as_str().parse().unwrap();
    let timestamp = NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, 0);

    let event: Event = captures.name("e").unwrap().as_str().parse().unwrap();

    return Ok(Log { timestamp, event });
  }
}

struct Guard {
  id: i32,
  total_sleep_minutes: i64,
  sleep_frequencies: [u32; 60],
}

impl Guard {
  pub fn new(id: i32) -> Guard {
    Guard {
      id,
      total_sleep_minutes: 0,
      sleep_frequencies: [0; 60],
    }
  }

  pub fn update_sleep_frequencies(&mut self, start: &NaiveDateTime, end: &NaiveDateTime) {
    let start_minute = start.time().minute() as usize;
    let end_minute = end.time().minute() as usize;
    for i in start_minute..end_minute {
      self.sleep_frequencies[i] += 1;
    }
  }
}

impl Eq for Guard {}

impl PartialEq for Guard {
  fn eq(&self, other: &Guard) -> bool {
    return self.id == other.id;
  }
}

impl Ord for Guard {
  fn cmp(&self, other: &Guard) -> Ordering {
    return self.total_sleep_minutes.cmp(&other.total_sleep_minutes);
  }
}

impl PartialOrd for Guard {
  fn partial_cmp(&self, other: &Guard) -> Option<Ordering> {
    return Some(self.cmp(other));
  }
}
pub fn run(filename: &String) {
  let mut lines = super::input::read_lines(filename.to_string());
  lines.sort_unstable();
  let mut guards = HashMap::<i32, Guard>::new();
  let mut current_guard: i32 = 0;
  let mut current_sleep_start: NaiveDateTime = NaiveDateTime::from_timestamp(0, 0);
  for line in lines {
    let log: Log = line.parse().unwrap();
    match log.event {
      Event::BeginsShift(guard) => {
        current_guard = guard;
        if !guards.contains_key(&guard) {
          guards.insert(guard, Guard::new(guard));
        }
      }
      Event::FallsAsleep => {
        current_sleep_start = log.timestamp;
      }
      Event::WakesUp => {
        let total_sleep_minutes = log
          .timestamp
          .signed_duration_since(current_sleep_start)
          .num_minutes();
        let guard = guards.get_mut(&current_guard).unwrap();
        guard.total_sleep_minutes += total_sleep_minutes;
        guard.update_sleep_frequencies(&current_sleep_start, &log.timestamp);
      }
    }
  }
  let mut lazy_guards: Vec<&Guard> = guards.values().collect();
  lazy_guards.sort_unstable();
  let laziest_guard: &Guard = &lazy_guards.last().unwrap();
  let mut most_frequent_minute: usize = 0;
  let mut frequency: u32 = 0;
  for i in 0..60 {
    let current_frequency = laziest_guard.sleep_frequencies[i];
    if current_frequency > frequency {
      most_frequent_minute = i;
      frequency = current_frequency;
    }
  }
  println!(
    "laziest guard {} slept for {} minutes, most frequently at 00:{:02}. answer key: {}",
    laziest_guard.id,
    laziest_guard.total_sleep_minutes,
    most_frequent_minute,
    laziest_guard.id * most_frequent_minute as i32
  );

  let mut most_frequent_minute = 0;
  let mut highest_frequency = 0;
  let mut most_consistent_guard = 0;
  for guard in lazy_guards {
    for i in 0..60 {
      let current_frequency = guard.sleep_frequencies[i];
      if current_frequency > highest_frequency {
        most_frequent_minute = i;
        highest_frequency = current_frequency;
        most_consistent_guard = guard.id;
      }
    }
  }

  println!(
    "guard {} is most consistent, sleeping at 00:{:02} {} times. answer key: {}",
    most_consistent_guard,
    most_frequent_minute,
    highest_frequency,
    most_consistent_guard * most_frequent_minute as i32
  );
}
