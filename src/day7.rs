use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Graph {
  nodes: HashMap<String, Node>,
  visited: HashSet<String>,
}

impl Graph {
  fn new() -> Graph {
    return Graph {
      nodes: HashMap::<String, Node>::new(),
      visited: HashSet::<String>::new(),
    };
  }

  fn parse_node(&mut self, s: &str) {
    lazy_static! {
      static ref step_re: Regex =
        Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    }
    let captures = step_re.captures(&s).unwrap();
    let first_step = captures[1].to_string();
    if !self.nodes.contains_key(&first_step) {
      let node = Node::new(first_step.clone());
      self.nodes.insert(first_step.clone(), node);
    }
    let second_step = captures[2].to_string();
    if !self.nodes.contains_key(&second_step) {
      let node = Node::new(second_step.clone());
      self.nodes.insert(second_step.clone(), node);
    }
    {
      let first_node: &mut Node;
      first_node = self.nodes.get_mut(&first_step).unwrap();
      first_node.precedes.insert(second_step.clone());
    }
    {
      let second_node: &mut Node;
      second_node = self.nodes.get_mut(&second_step).unwrap();
      second_node.follows.insert(first_step.clone());
    }
  }

  fn find_first(&self) -> Vec<String> {
    let mut first = Vec::<String>::new();

    for (_, node) in self.nodes.iter() {
      if node.follows.is_empty() {
        first.push(node.name.clone());
      }
    }
    return first;
  }

  fn walk_followers(&mut self, name: &String) -> Vec<String> {
    self.visited.insert(name.clone());

    let mut followers = Vec::<String>::new();
    let node_opt = self.nodes.get(name);
    if node_opt.is_none() {
      return followers;
    }

    let node = node_opt.unwrap();
    for follower_name in &node.precedes {
      let mut follower_opt = self.nodes.get(&follower_name.clone());
      if follower_opt.is_none() {
        continue;
      }
      let follower = follower_opt.unwrap();
      if follower.follows.iter().all(|x| self.visited.contains(x)) {
        followers.push(follower.name.clone());
      }
    }
    return followers;
  }
}

struct Node {
  name: String,
  follows: HashSet<String>,
  precedes: HashSet<String>,
}

impl Node {
  pub fn new(name: String) -> Node {
    return Node {
      name: name,
      follows: HashSet::<String>::new(),
      precedes: HashSet::<String>::new(),
    };
  }
}

pub fn run1(filename: &String) {
  let lines = super::input::read_lines(filename.to_string());
  let mut graph = Graph::new();
  for line in lines {
    graph.parse_node(&line);
  }
  let mut upcoming = Vec::<String>::new();
  let mut order = String::new();
  {
    upcoming.extend_from_slice(&graph.find_first());
  }

  while !upcoming.is_empty() {
    upcoming.sort_unstable();
    upcoming.reverse();
    let ref next_name = upcoming.pop().unwrap();
    order.push_str(next_name.clone().as_str());

    for ref follower_name in graph.walk_followers(next_name) {
      upcoming.push(follower_name.clone());
    }
  }
  println!("order: {:}", order);
}
