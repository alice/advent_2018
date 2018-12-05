pub fn run(filename: &String) {
  //  let polymer = "dabAcCaCBAcCcaDA";
  let polymer = std::fs::read_to_string(filename).expect("Failed to read input");
  let mut units: Vec<char> = polymer.chars().collect();
  let mut before = units.len();
  let mut after = 0;
  while before != after {
    before = units.len();
    let mut previous_unit = '\n';
    let mut accumulator = Vec::<char>::new();
    units.iter_mut().fold(&mut accumulator, |vec, unit| {
      if unit.eq_ignore_ascii_case(&previous_unit)
        && ((unit.is_uppercase() && previous_unit.is_lowercase())
          || (unit.is_lowercase() && previous_unit.is_uppercase()))
      {
        previous_unit = '\n';
        return vec;
      }
      if previous_unit != '\n' {
        vec.push(previous_unit);
      }
      previous_unit = *unit;
      return vec;
    });
    if previous_unit != '\n' {
      accumulator.push(previous_unit);
    }
    after = accumulator.len();
    units = accumulator;
  }
  let s: String = units.iter().collect();
  println!("after folding: {:?}, {} units", s, s.len());
}
