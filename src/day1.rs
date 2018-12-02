use std::collections::HashSet;

pub fn run1(filename: &String) {
  let rows = super::input::read_lines(filename.to_string());

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

pub fn run2(filename: &String) {
  let rows = super::input::read_lines(filename.to_string());

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
