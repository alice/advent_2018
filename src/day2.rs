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
    let this_row = &rows[i];
    for j in (i + 1)..rows.len() {
      let that_row = &rows[j];
      let mut anomaly = None;
      for (k, (this, that)) in (1..).zip(this_row.chars().zip(that_row.chars())) {
        if this != that {
          if !anomaly.is_none() {
            anomaly = None;
            break;
          }
          anomaly = Some(k);
        }
      }
      if !anomaly.is_none() {
        let mut result = String::new();
        result.push_str(&this_row[0..anomaly.unwrap()]);
        result.push_str(&this_row[anomaly.unwrap() + 1..]);
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
