use std::collections::HashSet;

pub fn run1() {
  let rows = super::input::read_lines("input/day1.txt".to_string());

  let mut frequency = 0;
  for row in rows {
    let result = row.parse::<i32>();
    if !result.is_ok() {
      break;
    }
    let number = result.unwrap();
    frequency += number;
  }

  println!("frequency: {}", frequency);
}

pub fn run2() {
  let rows = super::input::read_lines("input/day1.txt".to_string());

  let mut frequency = 0;
  let mut seen = HashSet::<i32>::new();
  loop {
    for row in &rows {
      let result = row.parse::<i32>();
      if !result.is_ok() {
        break;
      }
      let number = result.unwrap();

      frequency += number;
      if seen.contains(&frequency) {
        println!("twice: {}", frequency);
        return;
      }
      seen.insert(frequency);
    }
  }
}
