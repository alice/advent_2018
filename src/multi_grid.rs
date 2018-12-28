use multimap::*;
use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub struct Coords {
  pub x: i32,
  pub y: i32,
}

pub struct MultiGrid<T: fmt::Display> {
  grid: MultiMap<Coords, T>,
  left: Option<i32>,
  right: Option<i32>,
  top: Option<i32>,
  bottom: Option<i32>,
}

impl<T: fmt::Display> MultiGrid<T> {
  pub fn new() -> MultiGrid<T> {
    MultiGrid {
      grid: MultiMap::<Coords, T>::new(),
      left: None,
      right: None,
      top: None,
      bottom: None,
    }
  }

  pub fn insert(&mut self, x: i32, y: i32, cell: T) {
    self.left = Some(match self.left {
      None => x,
      Some(left) => std::cmp::min(left, x),
    });
    self.right = Some(match self.right {
      None => x,
      Some(right) => std::cmp::max(right, x),
    });
    self.top = Some(match self.top {
      None => y,
      Some(top) => std::cmp::min(top, y),
    });
    self.bottom = Some(match self.bottom {
      None => y,
      Some(bottom) => std::cmp::max(bottom, y),
    });

    let coords = Coords { x, y };
    self.grid.insert(coords, cell);
  }

  pub fn get(&self, x: i32, y: i32) -> Option<&T> {
    let coords = Coords { x, y };
    return self.grid.get(&coords);
  }

  pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
    let coords = Coords { x, y };
    return self.grid.get_mut(&coords);
  }

  pub fn has_cell_at(&self, x: i32, y: i32) -> bool {
    let coords = Coords { x, y };
    return self.grid.contains_key(&coords);
  }

  pub fn iter(&self) -> Iter<Coords, T> {
    return self.grid.iter();
  }

  pub fn cells_mut(&mut self) -> Vec<&mut T> {
    let mut result = Vec::<&mut T>::new();
    for (_, cells) in self.grid.iter_all_mut() {
      for cell in cells {
        result.push(cell);
      }
    }
    return result;
  }

  pub fn width(&self) -> usize {
    if self.left.is_none() || self.right.is_none() {
      println!("no bounds");
      return 0;
    }
    return (self.right.unwrap() - self.left.unwrap()) as usize;
  }

  pub fn clear(&mut self) {
    self.grid.clear();
    self.left = None;
    self.right = None;
    self.top = None;
    self.bottom = None;
  }

  pub fn print(&self) {
    if self.left.is_none() || self.right.is_none() || self.top.is_none() || self.bottom.is_none() {
      println!("no bounds");
      return;
    }
    let width = self.right.unwrap() - self.left.unwrap();
    if width > 100 {
      return;
    }

    let mut cell_width: usize = 0;
    for (_, cell) in self.iter() {
      let rendered_cell = format!("{}", cell);
      cell_width = std::cmp::max(cell_width, rendered_cell.len());
    }
    for y in self.top.unwrap()..=self.bottom.unwrap() {
      let mut rendered_row = String::new();
      for x in self.left.unwrap()..=self.right.unwrap() {
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
