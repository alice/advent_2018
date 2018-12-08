use grid::Grid;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

struct Location {
  pub x: usize,
  pub y: usize,
  pub id: Option<String>,
  pub num_closest: u32,
  pub infinite: bool,
}

static mut NEXT_ID: usize = 0;
static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
impl FromStr for Location {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref coords_re: Regex = Regex::new(r"(?P<x>\d+), (?P<y>\d+)").unwrap();
    };

    let captures = coords_re.captures(s).unwrap();

    let x: usize = captures.name("x").unwrap().as_str().parse().unwrap();
    let y: usize = captures.name("y").unwrap().as_str().parse().unwrap();
    let mut id_num;
    unsafe {
      id_num = NEXT_ID;
      NEXT_ID += 1;
    }
    let mut id = String::new();

    let idx = id_num % ALPHABET.len();
    println!("idx: {}, letter: {}", idx, &ALPHABET[idx..=idx]);
    id.push_str(&ALPHABET[idx..=idx]);
    id_num = id_num / ALPHABET.len();
    while id_num > 0 {
      let idx = id_num % ALPHABET.len();
      println!("idx: {}, letter: {}", idx, &ALPHABET[idx..=idx]);
      id.push_str(&ALPHABET[idx..=idx]);
      id_num = id_num / ALPHABET.len();
    }
    return Ok(Location {
      x: x,
      y: y,
      id: Some(id),
      num_closest: 0,
      infinite: false,
    });
  }
}

impl fmt::Display for Location {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.id {
      None => write!(f, "."),
      Some(ref id) => write!(f, "{}", id),
    }
  }
}

trait Closest<T> {
  fn compute_closest_coords(&self, row: usize, col: usize) -> Option<(usize, usize)>;
  fn compute_closest(&self, row: usize, col: usize) -> Option<&T>;
  fn compute_closest_mut(&mut self, row: usize, col: usize) -> Option<&mut T>;
}

impl<T: fmt::Display> Closest<T> for Grid<T> {
  fn compute_closest_coords(&self, col: usize, row: usize) -> Option<(usize, usize)> {
    if self.has_cell_at(col, row) {
      return Some((col, row));
    }
    let mut closest: Option<(usize, usize)> = None;
    let mut min_distance = std::usize::MAX;
    for (coords, _) in self.iter() {
      let x = coords.x as i32;
      let y = coords.y as i32;
      let distance = ((x - col as i32).abs() + (y - row as i32).abs()) as usize;

      if distance < min_distance {
        min_distance = distance;
        closest = Some((x as usize, y as usize));
      } else if distance == min_distance {
        closest = None;
      }
    }
    return closest;
  }

  fn compute_closest_mut(&mut self, col: usize, row: usize) -> Option<&mut T> {
    let closest_coords_option = self.compute_closest_coords(col, row);
    if closest_coords_option.is_none() {
      return None;
    }
    let (x, y) = closest_coords_option.unwrap();
    return self.get_mut(x, y);
  }

  fn compute_closest(&self, col: usize, row: usize) -> Option<&T> {
    let closest_coords_option = self.compute_closest_coords(col, row);
    if closest_coords_option.is_none() {
      return None;
    }
    let (x, y) = closest_coords_option.unwrap();
    return self.get(x, y);
  }
}

fn print_grid_with_closest<T: fmt::Display>(grid: &Grid<T>) {
  let mut cell_width: usize = 0;
  for (_, cell) in grid.iter() {
    let rendered_cell = format!("{}", cell);
    cell_width = std::cmp::max(cell_width, rendered_cell.len());
  }
  for y in 0..grid.height() {
    let mut rendered_row = String::new();
    for x in 0..grid.width() {
      let cell = grid.get(x, y);
      let mut rendered_cell;
      if cell.is_none() {
        let closest_option: Option<&T> = grid.compute_closest(x, y);
        if closest_option.is_none() {
          rendered_cell = ".".repeat(cell_width);
        } else {
          rendered_cell = format!("{}", closest_option.unwrap()).to_lowercase();
        }
      } else {
        rendered_cell = format!("{}", cell.unwrap());
      }
      let padding = " ".repeat(cell_width - rendered_cell.len());
      rendered_row.push_str(&padding);
      rendered_row.push_str(&rendered_cell);
    }
    println!("{}", rendered_row);
  }
}

pub fn run1(filename: &String) {
  let lines = super::input::read_lines(filename.to_string());
  let mut grid = Grid::<Location>::new();
  for line in lines {
    let location: Location = line.parse().unwrap();
    grid.insert(location.x, location.y, location);
  }

  for row in 0..grid.height() {
    for col in 0..grid.width() {
      let mut on_edge = false;
      if row == 0 || row == grid.height() - 1 || col == 0 || col == grid.width() - 1 {
        on_edge = true;
      }
      let closest_option: Option<&mut Location> = grid.compute_closest_mut(col, row);
      if closest_option.is_none() {
        continue;
      }
      let closest = closest_option.unwrap();
      if on_edge {
        closest.infinite = true;
        closest.num_closest = 0;
      } else if !closest.infinite {
        closest.num_closest += 1;
      }
    }
  }

  let (_, most) = grid
    .iter()
    .max_by(|(_, location1), (_, location2)| location1.num_closest.cmp(&location2.num_closest))
    .unwrap();
  println!(
    "{:} has the most closest to it ({})",
    most, most.num_closest
  );
}
