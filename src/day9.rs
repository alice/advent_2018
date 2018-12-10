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

fn print_marbles2(player: &str, next: &Vec<usize>, current_marble: usize) {
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

    if marble == current_marble {
      out.push_str(&format!("({})", marble));
    } else {
      out.push_str(&format!("{}", marble));
    }

    marble = next[marble];
  }
  println!("{}", out);
}

pub fn run2() {
  let mut next: Vec<usize> = (0..LAST_MARBLE + 1).into_iter().collect();
  let mut prev: Vec<usize> = (0..LAST_MARBLE + 1).into_iter().collect();

  let mut players: [usize; NUM_PLAYERS] = [0; NUM_PLAYERS];
  let mut current_player = 1;
  let mut next_marble = 1;
  let mut current_marble = 0;

  while next_marble <= LAST_MARBLE {
    if next_marble % 23 == 0 {
      players[current_player] += next_marble;
      for _ in 0..7 {
        current_marble = prev[current_marble];
      }
      players[current_player] += current_marble;
      let anticlockwise = prev[current_marble];
      let clockwise = next[current_marble];
      prev[clockwise] = anticlockwise;
      next[anticlockwise] = clockwise;
      current_marble = clockwise;
    } else {
      current_marble = next[current_marble];
      let clockwise = next[current_marble];
      next[current_marble] = next_marble;
      next[next_marble] = clockwise;
      prev[clockwise] = next_marble;
      prev[next_marble] = current_marble;
      current_marble = next_marble;
    }
    next_marble += 1;
    current_player = (current_player + 1) % NUM_PLAYERS;
  }

  let max = players.into_iter().max().unwrap();
  println!("Max score is {}", max);
}
