const NUM_PLAYERS: usize = 424;
const LAST_MARBLE: usize = 7148200;

fn print_marbles(player: &str, marbles: &Vec<usize>, current_marble_index: usize) {
  let mut out = String::new();
  out.push_str(player);
  for i in 0..marbles.len() {
    if i > 0 {
      out.push_str(" ");
    }
    if i == current_marble_index {
      out.push_str(&format!("({})", marbles[i]));
    } else {
      out.push_str(&format!("{}", marbles[i]));
    }
  }
  println!("{}", out);
}

pub fn run1() {
  let mut marbles = Vec::<usize>::new();
  let mut players: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
  let mut current_player = 0;
  let mut next_marble = 0;
  let mut current_marble_index = 0;

  marbles.push(0);
  next_marble += 1;

  while next_marble <= LAST_MARBLE {
    if next_marble % 23 == 0 {
      players[current_player] += next_marble;
      current_marble_index = (current_marble_index + marbles.len() - 7) % marbles.len();
      let score = marbles.remove(current_marble_index);
      players[current_player] += score;
    } else {
      let next_marble_index = ((current_marble_index + 1) % marbles.len()) + 1;
      marbles.insert(next_marble_index, next_marble);
      current_marble_index = next_marble_index;
    }

    next_marble += 1;
    current_player = (current_player + 1) % NUM_PLAYERS;
  }

  let max = players.into_iter().max().unwrap();
  println!("Max score is {}", max);
}

struct Circle {
  prev: Vec<usize>,
  next: Vec<usize>,
  current_marble: usize,
}

impl Circle {
  fn new(last_marble: usize) -> Circle {
    Circle {
      next: (0..last_marble + 1).into_iter().collect(),
      prev: (0..last_marble + 1).into_iter().collect(),
      current_marble: 0,
    }
  }

  fn print(&self, player: &str) {
    let mut out = String::new();
    out.push_str(player);
    let mut marble = 0;
    let mut first = true;
    while marble != 0 || first {
      if !first {
        out.push_str(" ");
      } else {
        first = false;
      }

      if marble == self.current_marble {
        out.push_str(&format!("({})", marble));
      } else {
        out.push_str(&format!("{}", marble));
      }

      marble = self.next[marble];
    }
    println!("{}", out);
  }

  fn insert_after_current(&mut self, next_marble: usize) {
    let next_next = self.next[self.current_marble];
    self.next[self.current_marble] = next_marble;
    self.next[next_marble] = next_next;
    self.prev[next_next] = next_marble;
    self.prev[next_marble] = self.current_marble;
    self.current_marble = next_marble;
  }

  fn advance(&mut self) {
    self.current_marble = self.next[self.current_marble];
  }

  fn back(&mut self) {
    self.current_marble = self.prev[self.current_marble];
  }

  fn remove_current(&mut self) -> usize {
    let temp_prev = self.prev[self.current_marble];
    let temp_next = self.next[self.current_marble];
    self.prev[temp_next] = temp_prev;
    self.next[temp_prev] = temp_next;
    let removed = self.current_marble;
    self.current_marble = temp_next;
    return removed;
  }
}

pub fn run2() {
  let mut circle = Circle::new(LAST_MARBLE);
  let mut players: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
  let mut current_player = 1;
  let mut next_marble = 1;

  while next_marble <= LAST_MARBLE {
    if next_marble % 23 == 0 {
      players[current_player] += next_marble;
      for _ in 0..7 {
        circle.back();
      }
      players[current_player] += circle.remove_current();
    } else {
      circle.advance();
      circle.insert_after_current(next_marble);
    }
    next_marble += 1;
    current_player = (current_player + 1) % NUM_PLAYERS;
    //circle.print(format!("[{}]  ", current_player).as_str());
  }

  let max = players.into_iter().max().unwrap();
  println!("Max score is {}", max);
}
