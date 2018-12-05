fn reduce(units: &mut Vec<char>) {
  let mut accumulator = Vec::<char>::new();
  units.iter_mut().fold(&mut accumulator, |vec, unit| {
    let last_unit = vec.pop();
    if last_unit.is_some()
      && unit.eq_ignore_ascii_case(&last_unit.unwrap())
      && ((unit.is_uppercase() && last_unit.unwrap().is_lowercase())
        || (unit.is_lowercase() && last_unit.unwrap().is_uppercase()))
    {
      return vec;
    }
    if last_unit.is_some() {
      vec.push(last_unit.unwrap());
    }
    vec.push(*unit);
    return vec;
  });
  *units = accumulator;
}

pub fn run1(filename: &String) {
  //let polymer = "dabAcCaCBAcCcaDA";
  let polymer = std::fs::read_to_string(filename).expect("Failed to read input");
  let mut units: Vec<char> = polymer.chars().collect();
  reduce(&mut units);
  let s: String = units.iter().collect();
  println!("after folding: {:?}, {} units", s, s.len());
}

pub fn run2(filename: &String) {
  //let polymer = "dabAcCaCBAcCcaDA";
  let mut polymer = std::fs::read_to_string(filename).expect("Failed to read input");
  polymer = polymer.trim().to_string();
  let units: Vec<char> = polymer.chars().collect();
  let mut unit_types = units.clone();
  unit_types.sort_unstable();
  unit_types.dedup();
  unit_types = unit_types
    .into_iter()
    .map(|c| c.to_uppercase().last().unwrap())
    .collect();
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
