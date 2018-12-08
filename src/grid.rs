use std::collections::hash_map::{Iter, IterMut};
use std::collections::HashMap;
use std::default::Default;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coords {
  pub x: usize,
  pub y: usize,
}

pub struct Grid<T: fmt::Display> {
  grid: HashMap<Coords, T>,
  width: usize,
  height: usize,
}

impl<T: fmt::Display> Grid<T> {
  pub fn new() -> Grid<T> {
    Grid {
      grid: HashMap::<Coords, T>::new(),
      width: Default::default(),
      height: Default::default(),
    }
  }

  pub fn insert(&mut self, x: usize, y: usize, cell: T) {
    self.width = std::cmp::max(self.width, x + 1);
    self.height = std::cmp::max(self.width, y + 1);
    let coords = Coords { x, y };
    self.grid.insert(coords, cell);
  }

  pub fn get(&self, x: usize, y: usize) -> Option<&T> {
    let coords = Coords { x, y };
    return self.grid.get(&coords);
  }

  pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
    let coords = Coords { x, y };
    return self.grid.get_mut(&coords);
  }

  pub fn has_cell_at(&self, x: usize, y: usize) -> bool {
    let coords = Coords { x, y };
    return self.grid.contains_key(&coords);
  }

  pub fn iter(&self) -> Iter<Coords, T> {
    return self.grid.iter();
  }

  pub fn iter_mut(&mut self) -> IterMut<Coords, T> {
    return self.grid.iter_mut();
  }

  pub fn width(&self) -> usize {
    return self.width;
  }

  pub fn height(&self) -> usize {
    return self.height;
  }

  pub fn print(&self) {
    let mut cell_width: usize = 0;
    for (_, cell) in &self.grid {
      let rendered_cell = format!("{}", cell);
      cell_width = std::cmp::max(cell_width, rendered_cell.len());
    }
    for y in 0..self.height {
      let mut rendered_row = String::new();
      for x in 0..self.width {
        let coords = Coords { x, y };
        let cell = self.grid.get(&coords);
        let mut rendered_cell;
        if cell.is_none() {
          rendered_cell = ".".repeat(cell_width);
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
}
