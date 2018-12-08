use multimap::MultiMap;
use regex::Regex;
use std::collections::{HashMap, HashSet};

static ALPHABET: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
static MIN_JOB_LENGTH: usize = 60;
static NUM_WORKERS: usize = 5;

fn compute_job_lengths() -> HashMap<String, usize> {
  let mut job_lengths = HashMap::<String, usize>::new();
  for (i, ch) in ALPHABET.char_indices() {
    job_lengths.insert(ch.to_string(), i + MIN_JOB_LENGTH + 1);
  }
  return job_lengths;
}
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

  fn runtime(&self, name: &String) -> usize {
    let node = self.nodes.get(name).unwrap();
    return node.runtime;
  }

  fn parse_node(&mut self, s: &str) {
    lazy_static! {
      static ref step_re: Regex =
        Regex::new(r"Step (\w) must be finished before step (\w) can begin.").unwrap();
    }
    lazy_static! {
      static ref job_lengths: HashMap<String, usize> = compute_job_lengths();
    }
    let captures = step_re.captures(&s).unwrap();
    let first_step = captures[1].to_string();
    if !self.nodes.contains_key(&first_step) {
      let node = Node::new(first_step.clone(), job_lengths[&first_step]);
      self.nodes.insert(first_step.clone(), node);
    }
    let second_step = captures[2].to_string();
    if !self.nodes.contains_key(&second_step) {
      let node = Node::new(second_step.clone(), job_lengths[&second_step]);
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
  runtime: usize,
}

impl Node {
  pub fn new(name: String, length: usize) -> Node {
    return Node {
      name: name,
      follows: HashSet::<String>::new(),
      precedes: HashSet::<String>::new(),
      runtime: length,
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

pub fn run2(filename: &String) {
  let lines = super::input::read_lines(filename.to_string());
  let mut graph = Graph::new();
  for line in lines {
    graph.parse_node(&line);
  }

  let mut upcoming = Vec::<String>::new();
  let mut in_progress = MultiMap::<usize, String>::new();
  let mut num_in_progress = 0;
  let mut current_time: usize = 0;
  let mut order = String::new();
  {
    upcoming.extend_from_slice(&graph.find_first());
  }

  while !upcoming.is_empty() || num_in_progress > 0 {
    upcoming.sort_unstable();
    upcoming.reverse();
    // Fill work queue
    while num_in_progress < NUM_WORKERS && !upcoming.is_empty() {
      let ref next_name = upcoming.pop().unwrap();
      order.push_str(next_name.clone().as_str());
      let runtime = graph.runtime(next_name);
      in_progress.insert(current_time + runtime, next_name.clone());
      num_in_progress += 1;
    }

    // Advance time to next event completion
    {
      let mut completion_times: Vec<&usize> = in_progress.keys().collect();
      completion_times.sort_unstable();
      current_time = *completion_times[0];
    }

    // Fill upcoming queue from completed tasks.
    let completed_tasks: Option<Vec<String>> = in_progress.remove(&current_time);
    for completed_task in completed_tasks.unwrap() {
      num_in_progress -= 1;
      for ref follower_name in graph.walk_followers(&completed_task) {
        upcoming.push(follower_name.clone());
      }
    }
  }
  println!("order: {:}, current_time: {}", order, current_time);
}
