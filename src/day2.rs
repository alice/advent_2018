use std::iter::FromIterator;

pub fn run1() {
  let rows = super::input::read_lines("input/day2.txt".to_string());

  let mut twins = 0;
  let mut triplets = 0;
  for row in rows {
    let mut chars: Vec<char> = row.chars().collect();
    chars.sort_unstable();
    let mut consecutive = 1;
    let mut found_twins = false;
    let mut found_triplets = false;
    let mut last_char: char = chars[0];
    let mut i = 1;
    while i < chars.len() {
      let mut char = chars[i];
      if char == last_char {
        consecutive = consecutive + 1;
      } else {
        if consecutive == 2 {
          found_twins = true;
        }
        if consecutive == 3 {
          found_triplets = true;
        }
        consecutive = 1;
      }
      last_char = char;
      i = i + 1;
    }
    // handle last character
    if consecutive == 2 {
      found_twins = true;
    }
    if consecutive == 3 {
      found_triplets = true;
    }

    if found_twins {
      twins = twins + 1;
    }
    if found_triplets {
      triplets = triplets + 1;
    }
  }
  println!(
    "twins: {}, triplets: {}, checksum: {}",
    twins,
    triplets,
    twins * triplets
  );
}

pub fn run2() {
  let rows = super::input::read_lines("input/day2.txt".to_string());
  let mut i = 0;
  let mut j;
  while i < rows.len() {
    let this_row: Vec<char> = rows[i].chars().collect();
    j = i + 1;
    while j < rows.len() {
      let that_row: Vec<char> = rows[j].chars().collect();
      let mut k = 0;
      let mut diff = 0;
      let mut common = Vec::new();
      while k < this_row.len() && k < that_row.len() {
        if this_row[k] != that_row[k] {
          diff = diff + 1;
          if diff > 1 {
            break;
          }
        } else {
          common.push(this_row[k]);
        }
        k = k + 1;
      }
      if diff == 1 {
        let result = String::from_iter(common.iter());
        println!(
          "Found near match: {} and {}. Common: {}",
          rows[i], rows[j], result
        );
        return;
      }
      j = j + 1;
    }
    i = i + 1;
  }
  println!("Uh oh, no match found :(");
}
