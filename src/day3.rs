use grid::Grid;
use regex::Regex;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

struct Inch {
  id: i32,
  multiple: bool,
}

impl fmt::Display for Inch {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.multiple {
      return write!(f, "X");
    }
    return write!(f, "{}", self.id);
  }
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
    lazy_static! {
      static ref claim_re: Regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    };
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

pub fn run1(filename: &String) {
  let mut non_overlapped = HashSet::<i32>::new();
  let mut overlapping = 0;
  let mut grid = Grid::<Inch>::new();

  let lines = super::input::read_lines(filename.to_string());
  for line in lines {
    let claim: Claim = line.parse().unwrap();
    non_overlapped.insert(claim.id);
    for x in claim.left..claim.left + claim.width {
      for y in claim.top..claim.top + claim.height {
        if grid.has_cell_at(x, y) {
          let mut inch = grid.get_mut(x, y).unwrap();
          if !inch.multiple {
            inch.multiple = true;
            overlapping += 1;
          }
          non_overlapped.remove(&inch.id);
          non_overlapped.remove(&claim.id);
        } else {
          let inch = Inch {
            id: claim.id,
            multiple: false,
          };
          grid.insert(x, y, inch);
        }
      }
    }
  }

  //grid.print();
  println!(
    "overlapping: {}, not overlapped: {:?}",
    overlapping, non_overlapped
  )
}
