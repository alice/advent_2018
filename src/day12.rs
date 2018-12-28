use regex::Regex;
use std::collections::{BTreeMap, HashSet};

const INITIAL_STATE: &str = "#.#..#..###.###.#..###.#####...########.#...#####...##.#....#.####.#.#..#..#.#..###...#..#.#....##";
//const INITIAL_STATE: &str = "#..#.#..##......###...###";
const FIVE_DIGITS: usize = 1 << 5;
const FINAL_GENERATION: i64 = 50000000000;

fn print_state(state: &BTreeMap<i32, bool>, generation: usize, leftmost: i32) {
  let mut result = 0;
  let mut debug = "".to_string();
  for (n, plant) in state.clone() {
    match n {
      0 => debug.push('['),
      _ => {}
    }
    match plant {
      true => {
        debug.push('#');
        result += n
      }
      false => debug.push('.'),
    }
    match n {
      0 => debug.push(']'),
      _ => {}
    }
  }
  println!("{} ({}/{}): {}", generation, leftmost, result, debug);
}

fn print_state_vec(state: &Vec<bool>, generation: usize, leftmost: i64) {
  let mut result = 0;
  let mut debug = "".to_string();
  for (n, plant) in (leftmost..).zip(state.clone()) {
    match n {
      0 => debug.push('['),
      _ => {}
    }
    match plant {
      true => {
        debug.push('#');
        result += n
      }
      false => debug.push('.'),
    }
    match n {
      0 => debug.push(']'),
      _ => {}
    }
  }
  println!("{} ({}/{}): {}", generation, leftmost, result, debug);
}

fn print_digits(prefix: &str, number: u64) {
  let mut n = number;
  let mut debug = "".to_string();
  while n > 0 {
    match n % 2 {
      0 => debug.insert(0, '.'),
      1 => debug.insert(0, '#'),
      _ => debug.insert(0, '?'),
    }
    n >>= 1;
  }
  println!("{}: {}", prefix, debug);
}

pub fn run1(filename: &String) {
  let mut plants = BTreeMap::<i32, bool>::new();
  let mut generators = HashSet::<usize>::new();
  let mut leftmost: i32 = 0;
  let mut rightmost: i32 = 0;

  for (n, ch) in INITIAL_STATE.chars().enumerate() {
    match ch {
      '#' => plants.insert(n as i32, true),
      _ => plants.insert(n as i32, false),
    };
    rightmost = n as i32;
  }

  lazy_static! {
    static ref line_re: Regex = Regex::new(r"([\.#]{5}) => ([\.#])").unwrap();
  };
  let lines = super::input::read_lines(filename.to_string());
  for line in lines {
    let captures = line_re.captures(line.as_str()).unwrap();
    let pattern = captures.get(1).unwrap().as_str();
    let result = captures.get(2).unwrap().as_str();
    if result == "." {
      continue;
    }
    let mut generator = 0;
    for ch in pattern.chars() {
      generator <<= 1;
      match ch {
        '#' => generator += 1,
        _ => {}
      }
    }
    generators.insert(generator);
  }

  print_state(&plants, 0, leftmost);
  let mut result = 0;
  for (n, plant) in plants.clone() {
    if plant {
      result += n;
    }
  }
  println!("current total: {}", result);
  for generation in 1..=20 {
    let mut new_plants = BTreeMap::<i32, bool>::new();
    let mut last_five: usize = 0;
    for n in (leftmost as i32 - 3)..=(rightmost as i32 + 3) {
      last_five <<= 1;
      last_five %= FIVE_DIGITS;

      let mut plant = false;
      if n >= leftmost && n <= rightmost {
        plant = *plants.get(&n).unwrap();
      }
      if plant {
        last_five += 1;
      }

      let new_plant = n - 2;
      if generators.contains(&last_five) {
        new_plants.insert(new_plant, true);
        if new_plant < leftmost {
          leftmost = new_plant;
        } else if new_plant > rightmost {
          rightmost = new_plant;
        }
      } else if new_plant >= leftmost && new_plant <= rightmost {
        new_plants.insert(new_plant, false);
      }
    }
    print_state(&new_plants, generation, leftmost);
    let mut result = 0;
    for (n, plant) in plants {
      if plant {
        result += n;
      }
    }
    println!("current total: {}", result);
    plants = new_plants;
  }
  let mut result = 0;
  for (n, plant) in plants {
    if plant {
      result += n;
    }
  }

  println!("result: {}", result);
}

pub fn run2(filename: &String) {
  let mut plants = Vec::<bool>::new();
  let mut generators = HashSet::<usize>::new();
  let mut leftmost: i64 = 0;

  for ch in INITIAL_STATE.chars() {
    match ch {
      '#' => plants.push(true),
      _ => plants.push(false),
    };
  }

  lazy_static! {
    static ref line_re: Regex = Regex::new(r"([\.#]{5}) => ([\.#])").unwrap();
  };
  let lines = super::input::read_lines(filename.to_string());
  for line in lines {
    let captures = line_re.captures(line.as_str()).unwrap();
    let pattern = captures.get(1).unwrap().as_str();
    let result = captures.get(2).unwrap().as_str();
    if result == "." {
      continue;
    }
    let mut generator = 0;
    for ch in pattern.chars() {
      generator <<= 1;
      match ch {
        '#' => generator += 1,
        _ => {}
      }
    }
    generators.insert(generator);
  }

  let mut generation = 1;
  let mut leftmost_delta;
  loop {
    let mut last_five: usize = 0;
    let mut new_plants = Vec::<bool>::new();
    let rightmost = plants.len() as i64 + leftmost;
    let mut new_leftmost = leftmost;
    let mut found_first = false;
    for n in (leftmost - 3)..(rightmost + 3) {
      last_five <<= 1;
      last_five %= FIVE_DIGITS;

      let mut plant = false;
      if n >= leftmost && n < rightmost {
        plant = plants[(n - leftmost) as usize];
      }
      if plant {
        last_five += 1;
      }

      let i = n - 2;
      if generators.contains(&last_five) {
        new_plants.push(true);
        if !found_first {
          new_leftmost = i;
          found_first = true;
        }
      } else if found_first && i < rightmost {
        new_plants.push(false);
      }
    }

    leftmost_delta = new_leftmost - leftmost;
    leftmost = new_leftmost;
    if new_plants == plants {
      break;
    }
    plants = new_plants;
    print_state_vec(&plants, generation, leftmost);
    generation += 1;
  }

  let generation_delta: i64 = FINAL_GENERATION - generation as i64;
  leftmost = leftmost + (generation_delta * leftmost_delta);

  let mut result = 0;
  for (n, plant) in (leftmost..).zip(plants) {
    if plant {
      result += n;
    }
  }

  println!("result: {}", result);
}
