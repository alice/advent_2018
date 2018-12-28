use grid::{Coords, Direction, Grid, Turn};
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

#[derive(Eq, PartialEq, Hash)]
struct Cart {
  id: usize,
  direction: Direction,
  location: Coords,
  next_turn: Turn,
}

static mut NEXT_ID: usize = 0;
impl Cart {
  pub fn new(direction: Direction, location: Coords) -> Cart {
    unsafe {
      NEXT_ID += 1;

      Cart {
        id: NEXT_ID,
        direction,
        location,
        next_turn: Turn::Left,
      }
    }
  }
}

#[derive(Clone)]
enum PathType {
  Horizontal,
  Vertical,
  Intersection,
  CurveRight,
  CurveLeft,
  Space,
}

#[derive(Clone)]
struct Segment {
  path_type: PathType,
  direction: Option<Direction>,
  cart: Option<usize>,
}

impl Segment {
  pub fn new(path_type: PathType, direction: Option<Direction>) -> Segment {
    Segment {
      path_type,
      direction,
      cart: None,
    }
  }
}

impl fmt::Display for Segment {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::PathType::*;
    use grid::Direction::*;

    let mut s = match self.path_type {
      Horizontal => '-',
      Vertical => '|',
      Intersection => '+',
      CurveLeft => '\\',
      CurveRight => '/',
      Space => ' ',
    };
    match self.direction {
      None => {}
      Some(cart) => {
        s = match cart {
          Left => '<',
          Right => '>',
          Up => '^',
          Down => 'v',
        }
      }
    }
    write!(f, "{}", s)
  }
}

#[derive(Debug)]
struct SegmentParseError {}

impl FromStr for Segment {
  type Err = SegmentParseError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    use self::PathType::*;
    use grid::Direction::*;
    match s {
      "-" => {
        return Ok(Segment::new(Horizontal, None));
      }
      "|" => {
        return Ok(Segment::new(Vertical, None));
      }
      "+" => {
        return Ok(Segment::new(Intersection, None));
      }
      "\\" => {
        return Ok(Segment::new(CurveLeft, None));
      }
      "/" => {
        return Ok(Segment::new(CurveRight, None));
      }
      ">" => {
        return Ok(Segment::new(Horizontal, Some(Right)));
      }
      "<" => {
        return Ok(Segment::new(Horizontal, Some(Left)));
      }
      "^" => {
        return Ok(Segment::new(Vertical, Some(Up)));
      }
      "v" => {
        return Ok(Segment::new(Vertical, Some(Down)));
      }
      " " => {
        return Ok(Segment::new(Space, None));
      }
      _ => return Err(SegmentParseError {}),
    }
  }
}

struct Track {
  grid: Grid<Segment>,
  pub carts: Vec<Cart>,
}
impl Track {
  fn new() -> Track {
    return Track {
      grid: Grid::<Segment>::new(),
      carts: Vec::<Cart>::new(),
    };
  }

  pub fn from_file(filename: &String) -> Track {
    let mut track = Track::new();
    let lines = super::input::read_lines(filename.to_string());
    for (y, line) in (0..).zip(lines) {
      for (x, char) in line.chars().enumerate() {
        let coords = Coords { x, y };
        let mut segment: Segment = char.to_string().parse().unwrap();
        match segment.direction {
          None => {}
          Some(direction) => {
            let mut cart = Cart::new(direction, coords);
            track.carts.push(cart);
          }
        }
        track.add_segment(segment, coords);
      }
    }
    return track;
  }

  pub fn print(&self) {
    self.grid.print();
  }

  fn add_segment(&mut self, segment: Segment, coords: Coords) {
    self.grid.insert_at(coords, segment);
  }

  pub fn clear(&mut self, location: &Coords) {
    let segment = self.grid.get_at_mut(*location).unwrap();
    segment.cart = None;
    segment.direction = None;
  }

  pub fn tick(&mut self) -> Vec<Coords> {
    use self::PathType::*;
    use grid::Direction::*;

    let mut result = Vec::<Coords>::new();
    let mut crashed_carts = HashSet::<usize>::new();
    for ref mut cart in self.carts.iter_mut() {
      if crashed_carts.contains(&cart.id) {
        continue;
      }
      {
        let segment = self.grid.get_at_mut(cart.location).unwrap();
        segment.cart = None;
        segment.direction = None;
      }
      let direction = cart.direction;
      let next_location = self.grid.get_offset(cart.location, 1, direction);
      cart.location = next_location;
      let mut next_segment = self.grid.get_at_mut(next_location).unwrap();

      match next_segment.cart {
        None => {
          match &next_segment.path_type {
            CurveRight => {
              cart.direction = match direction {
                Up => Right,
                Left => Down,
                Down => Left,
                Right => Up,
              };
            }
            CurveLeft => {
              cart.direction = match direction {
                Up => Left,
                Left => Up,
                Down => Right,
                Right => Down,
              };
            }
            Intersection => {
              cart.direction = direction.turn(cart.next_turn);
              cart.next_turn = match cart.next_turn {
                Turn::Left => Turn::Straight,
                Turn::Straight => Turn::Right,
                Turn::Right => Turn::Left,
              };
            }
            _ => {}
          };
          next_segment.cart = Some(cart.id);
          next_segment.direction = Some(cart.direction);
        }
        Some(cart_id) => {
          crashed_carts.insert(cart_id);
          crashed_carts.insert(cart.id);
          next_segment.cart = None;
          next_segment.direction = None;
          result.push(next_location);
        }
      };
    }
    self.carts.retain(|cart| !crashed_carts.contains(&cart.id));
    return result;
  }
}

pub fn run1(filename: &String) {
  let mut track = Track::from_file(filename);
  let mut crashes = Vec::<Coords>::new();
  while crashes.is_empty() {
    crashes = track.tick();
  }
  println!(
    "Crash at {},{}",
    crashes.first().unwrap().x,
    crashes.first().unwrap().y
  );
}

pub fn run2(filename: &String) {
  let mut track = Track::from_file(filename);
  let mut remaining_carts = 0;
  while track.carts.len() > 1 {
    let crashes = track.tick();
    for crash in crashes {
      println!("Crash at {},{}", crash.x, crash.y);
    }
    if track.carts.len() != remaining_carts {
      remaining_carts = track.carts.len();
      println!("Remaining carts: {}", remaining_carts);
    }
  }
  let last_cart = track.carts.first().unwrap();
  println!(
    "Final cart at {},{}",
    last_cart.location.x, last_cart.location.y
  );
}
