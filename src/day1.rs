use std::collections::HashSet;

pub fn run1(filename: &String) {
  let rows = super::input::read_lines(filename.to_string());

  let frequency = rows
    .iter()
    .map(|row| row.parse::<i32>().unwrap())
    .fold(0, |acc, x| acc + x);

  println!("frequency: {}", frequency);
}

pub fn run2(filename: &String) {
  let rows = super::input::read_lines(filename.to_string());
  let deltas: Vec<i32> = rows.iter().map(|row| row.parse::<i32>().unwrap()).collect();

  let mut frequency = 0;
  let mut seen = HashSet::<i32>::new();
  loop {
    for delta in &deltas {
      frequency += delta;
      if seen.contains(&frequency) {
        println!("twice: {}", frequency);
        return;
      }
      seen.insert(frequency);
    }
  }
}
