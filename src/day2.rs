use std::iter::FromIterator;

pub fn run1(filename: &String) {
  let rows = super::input::read_lines(filename.to_string());

  let mut twins = 0;
  let mut triplets = 0;
  for row in rows {
    let mut chars: Vec<char> = row.chars().collect();
    chars.sort_unstable();
    chars.push('\n');
    let mut consecutive = 1;
    let mut found_twins = false;
    let mut found_triplets = false;
    let mut last_char = chars[0];
    for i in 1..chars.len() {
      let mut char = chars[i];
      if char == last_char {
        consecutive += 1;
      } else {
        match consecutive {
          2 => found_twins = true,
          3 => found_triplets = true,
          _ => {}
        }
        consecutive = 1;
      }
      last_char = char;
    }
    twins += found_twins as i32;
    triplets += found_triplets as i32;
  }
  println!("checksum: {}", twins * triplets);
}

pub fn run2(filename: &String) {
  let rows = super::input::read_lines(filename.to_string());
  for i in 0..rows.len() {
    let this_row: Vec<char> = rows[i].chars().collect();
    for j in (i + 1)..rows.len() {
      let that_row: Vec<char> = rows[j].chars().collect();
      let mut diff = 0;
      let mut common = Vec::new();
      // assuming same length rows
      for k in 0..this_row.len() {
        if this_row[k] != that_row[k] {
          diff = diff + 1;
          if diff > 1 {
            break;
          }
        } else {
          common.push(this_row[k]);
        }
      }
      if diff == 1 {
        let result = String::from_iter(common.iter());
        println!(
          "Found near match: {} and {}. Common: {}",
          rows[i], rows[j], result
        );
        return;
      }
    }
  }
  println!("Uh oh, no match found :(");
}
