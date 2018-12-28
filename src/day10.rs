use multi_grid::*;
use regex::Regex;
use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

#[derive(Debug, Clone)]
struct Velocity {
  x: i32,
  y: i32,
}

#[derive(Debug, Clone)]
struct Point {
  position: Coords,
  velocity: Velocity,
}

trait Tick {
  fn tick(&mut self);
  fn untick(&mut self);
}

impl Tick for Point {
  fn tick(&mut self) {
    self.position.x += self.velocity.x;
    self.position.y += self.velocity.y;
  }

  fn untick(&mut self) {
    self.position.x -= self.velocity.x;
    self.position.y -= self.velocity.y;
  }
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "#")
  }
}

impl FromStr for Point {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    lazy_static! {
      static ref point_re: Regex = Regex::new(
        r"position=< ?(?P<x>-?\d+), +(?P<y>-?\d+)> velocity=< ?(?P<xx>-?\d+), +(?P<yy>-?\d+)>"
      )
      .unwrap();
    };
    let captures = point_re.captures(s).unwrap();

    let x: i32 = captures.name("x").unwrap().as_str().parse().unwrap();
    let y: i32 = captures.name("y").unwrap().as_str().parse().unwrap();
    let velocity_x = captures.name("xx").unwrap().as_str().parse().unwrap();
    let velocity_y = captures.name("yy").unwrap().as_str().parse().unwrap();
    return Ok(Point {
      position: Coords { x: x, y: y },
      velocity: Velocity {
        x: velocity_x,
        y: velocity_y,
      },
    });
  }
}

impl Tick for MultiGrid<Point> {
  fn tick(&mut self) {
    let mut new_points = Vec::<Point>::new();
    for ref mut point in self.cells_mut() {
      point.tick();
      new_points.push(point.clone());
    }
    self.clear();
    for point in new_points {
      self.insert(point.position.x, point.position.y, point);
    }
  }

  fn untick(&mut self) {
    let mut new_points = Vec::<Point>::new();
    for ref mut point in self.cells_mut() {
      point.untick();
      new_points.push(point.clone());
    }
    self.clear();
    for point in new_points {
      self.insert(point.position.x, point.position.y, point);
    }
  }
}

pub fn run(filename: &String) {
  let lines = super::input::read_lines(filename.to_string());
  let mut grid = MultiGrid::<Point>::new();
  for line in lines {
    let point: Point = line.parse().unwrap();
    grid.insert(point.position.x, point.position.y, point);
  }
  let mut time: usize = 0;
  let mut last_width = grid.width();
  grid.tick();
  time += 1;
  while grid.width() < last_width {
    last_width = grid.width();
    grid.tick();
    time += 1;
  }
  grid.untick();
  time -= 1;
  grid.print();
  println!("Converged after {} seconds", time);
}
