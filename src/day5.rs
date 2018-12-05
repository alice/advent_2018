fn reduce(units: &mut Vec<char>) {
  let mut accumulator = Vec::<char>::new();
  units.iter_mut().fold(&mut accumulator, |vec, unit| {
    let maybe_last_unit = vec.pop();
    if maybe_last_unit.is_none() {
      vec.push(*unit);
      return vec;
    }
    let last_unit = maybe_last_unit.unwrap();
    if unit.eq_ignore_ascii_case(&last_unit)
      && ((unit.is_uppercase() && last_unit.is_lowercase())
        || (unit.is_lowercase() && last_unit.is_uppercase()))
    {
      return vec;
    }
    vec.push(last_unit);
    vec.push(*unit);
    return vec;
  });
  *units = accumulator;
}

pub fn run1(filename: &String) {
  //let polymer = "dabAcCaCBAcCcaDA";
  let mut polymer = std::fs::read_to_string(filename).expect("Failed to read input");
  polymer = polymer.trim().to_string();
  let mut units: Vec<char> = polymer.chars().collect();
  reduce(&mut units);
  let s: String = units.iter().collect();
  println!("after folding: {} units", s.len());
}

pub fn run2(filename: &String) {
  //let polymer = "dabAcCaCBAcCcaDA";
  let mut polymer = std::fs::read_to_string(filename).expect("Failed to read input");
  polymer = polymer.trim().to_string();
  let units: Vec<char> = polymer.chars().collect();
  let mut unit_types = units.clone();
  unit_types = unit_types
    .into_iter()
    .map(|c| c.to_uppercase().last().unwrap())
    .collect();
  unit_types.sort_unstable();
  unit_types.dedup();
  let mut best = '?';
  let mut shortest = std::usize::MAX;
  for unit_type in unit_types {
    let mut clone = units.clone();
    clone.retain(|&u| !u.eq_ignore_ascii_case(&unit_type));
    reduce(&mut clone);
    if clone.len() < shortest {
      best = unit_type;
      shortest = clone.len();
    }
  }
  println!("Best was {}, length of {}", best, shortest);
}
