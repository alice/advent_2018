use grid::{Coords, Grid};
use std::fmt;

const SERIAL_NUMBER: i32 = 9810;
const GRID_WIDTH: usize = 300;
const GRID_HEIGHT: usize = GRID_WIDTH;

#[derive(Debug, Clone)]
pub struct PowerCell {
  pub coords: Coords,
}

impl PowerCell {
  pub fn new(x: usize, y: usize) -> PowerCell {
    PowerCell {
      coords: Coords { x, y },
    }
  }

  pub fn power_level(&self) -> i32 {
    let rack_id: i32 = self.coords.x as i32 + 10;
    let mut power_level: i32 = rack_id * (self.coords.y as i32);
    power_level += SERIAL_NUMBER;
    power_level = power_level * rack_id;
    power_level = (power_level / 100) % 10;
    power_level = power_level - 5;
    return power_level;
  }
}

impl fmt::Display for PowerCell {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let power_level = self.power_level();
    let sign = match power_level < 0 {
      true => "-",
      false => " ",
    };
    return write!(f, "{}{} ", sign, power_level.abs());
  }
}

pub fn run1() {
  let mut grid = Grid::<PowerCell>::new();
  let mut squares = [[0i32; GRID_WIDTH]; GRID_HEIGHT];
  let mut max_square = Coords { x: 0, y: 0 };
  let mut max_power: i32 = 0;
  for y in 0..GRID_HEIGHT {
    for x in 0..GRID_WIDTH {
      let mut power_cell = PowerCell::new(x + 1, y + 1);
      let power_level: i32 = power_cell.power_level();
      grid.insert(power_cell.coords.x, power_cell.coords.y, power_cell);
      for dx in 0..3 {
        if dx > x {
          continue;
        }
        let xx: usize = x - dx;
        for dy in 0..3 {
          if dy > y {
            continue;
          }
          let yy: usize = y - dy;
          squares[xx][yy] += power_level;
          if dx == 2 && dy == 2 && squares[xx][yy] > max_power {
            max_power = squares[xx][yy];
            max_square.x = xx + 1;
            max_square.y = yy + 1;
          }
        }
      }
    }
  }
  println!(
    "max_square: {},{}, max_power: {}",
    max_square.x, max_square.y, max_power
  );
}

pub fn run2() {
  let mut max_square = Coords { x: 0, y: 0 };
  let mut max_power: i32 = 0;
  let mut optimal_size: usize = 0;
  let mut power_levels = [[0i32; GRID_WIDTH]; GRID_HEIGHT];
  let mut grid = Grid::<PowerCell>::new();
  for y in 0..GRID_HEIGHT {
    for x in 0..GRID_WIDTH {
      let mut power_cell = PowerCell::new(x + 1, y + 1);
      power_levels[y][x] = power_cell.power_level();
      grid.insert(power_cell.coords.x, power_cell.coords.y, power_cell);
    }
  }
  let mut squares = [[0i32; GRID_WIDTH]; GRID_HEIGHT];
  for size in 1..=GRID_HEIGHT {
    println!("size = {}", size);
    let start = size - 1;
    for y in start..GRID_HEIGHT {
      for x in start..GRID_WIDTH {
        // vertical edge
        let dx = size - 1;
        let xx: usize = x - dx;
        for dy in 0..size {
          let yy: usize = y - dy;
          squares[yy][xx] += power_levels[y][x];
          if dx == (size - 1) && dy == (size - 1) && squares[yy][xx] > max_power {
            max_power = squares[yy][xx];
            max_square.x = xx + 1;
            max_square.y = yy + 1;
            optimal_size = size;
            println!(
              "max_square: {},{},{} max_power: {}",
              max_square.x, max_square.y, optimal_size, max_power
            );
          }
        }

        // horizontal edge
        let dy = size - 1;
        let yy: usize = y - dy;
        for dx in 0..size - 1 {
          let xx: usize = x - dx;
          squares[yy][xx] += power_levels[y][x];
        }
      }
    }
  }
  println!(
    "max_square: {},{},{} max_power: {}",
    max_square.x, max_square.y, optimal_size, max_power
  );
}
