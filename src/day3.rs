extern crate math;
extern crate regex;

use self::math::round;
use self::regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;
use std::string::ParseError;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Coords {
  x: usize,
  y: usize,
}

struct Inch {
  id: i32,
  multiple: bool,
}

struct Claim {
  id: i32,
  left: usize,
  top: usize,
  width: usize,
  height: usize,
}

impl FromStr for Claim {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let claim_re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    let captures = claim_re.captures(s).unwrap();

    let id: i32 = captures[1].parse().unwrap();
    let left: usize = captures[2].parse().unwrap();
    let top: usize = captures[3].parse().unwrap();
    let width: usize = captures[4].parse().unwrap();
    let height: usize = captures[5].parse().unwrap();

    return Ok(Claim {
      id,
      left,
      top,
      width,
      height,
    });
  }
}

fn print_grid(grid: &HashMap<Coords, Inch>, width: usize, height: usize, max: i32) {
  let digits: usize = (round::floor((max as f64).log10(), 0) as usize) + 1;
  for y in 0..height {
    let mut rendered_row = String::new();
    for x in 0..width {
      let coords = Coords { x, y };
      let mut rendered_cell;
      if !grid.contains_key(&coords) {
        rendered_cell = ".".repeat(digits);
      } else {
        let inch = grid.get(&coords).unwrap();
        if inch.multiple {
          rendered_cell = "X".repeat(digits);
        } else {
          rendered_cell = match digits {
            2 => format!("{:02}", inch.id),
            3 => format!("{:03}", inch.id),
            4 => format!("{:04}", inch.id),
            5 => format!("{:05}", inch.id),
            _ => format!("{}", inch.id),
          };
        }
      }
      rendered_row.push_str(&rendered_cell);
    }
    println!("{}", rendered_row);
  }
}

pub fn run1(filename: &String) {
  let mut grid: HashMap<Coords, Inch> = HashMap::new();
  let mut non_overlapped = HashSet::<i32>::new();
  let mut overlapping = 0;

  let mut grid_width: usize = 0;
  let mut grid_height: usize = 0;
  let mut max: i32 = 0;

  let lines = super::input::read_lines(filename.to_string());

  for line in lines {
    let claim: Claim = line.parse().unwrap();
    non_overlapped.insert(claim.id);
    max = std::cmp::max(max, claim.id);
    grid_width = std::cmp::max(grid_width, claim.left + claim.width);
    grid_height = std::cmp::max(grid_height, claim.top + claim.height);
    for x in claim.left..claim.left + claim.width {
      for y in claim.top..claim.top + claim.height {
        let mut coords = Coords { x, y };
        if grid.contains_key(&coords) {
          let mut inch = grid.get_mut(&coords).unwrap();
          if !inch.multiple {
            inch.multiple = true;
            overlapping += 1;
          }
          non_overlapped.remove(&inch.id);
          non_overlapped.remove(&claim.id);
        } else {
          grid.insert(
            coords,
            Inch {
              id: claim.id,
              multiple: false,
            },
          );
        }
      }
    }
  }

  //print_grid(&grid, grid_width, grid_height, max);
  println!(
    "overlapping: {}, not overlapped: {:?}",
    overlapping, non_overlapped
  )
}
