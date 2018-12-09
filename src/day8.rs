fn parse_node(input: &[usize], next: &mut usize, metadata_accumulator: &mut usize, is_part2: bool) {
  let num_nodes = input[*next];
  *next += 1;
  let num_metadata: usize = input[*next];
  *next += 1;
  let mut child_metadatas = Vec::<usize>::new();
  for _ in 0..num_nodes {
    let mut child_metadata: usize = 0;
    parse_node(input, next, &mut child_metadata, is_part2);
    child_metadatas.push(child_metadata);
  }
  for _ in 0..num_metadata {
    let mut metadata_value: usize = input[*next];
    *next += 1;
    if num_nodes > 0 && is_part2 {
      if metadata_value > 0 && metadata_value <= num_nodes {
        metadata_value = child_metadatas[metadata_value - 1];
      } else {
        metadata_value = 0;
      }
    }
    *metadata_accumulator += metadata_value;
  }
  if !is_part2 {
    for child_metadata in child_metadatas {
      *metadata_accumulator += child_metadata;
    }
  }
}

pub fn run1(filename: &String) {
  let input = std::fs::read_to_string(filename).expect("Failed to read input");
  let numbers: Vec<usize> = input.split(" ").map(|x| x.parse().unwrap()).collect();
  let mut total: usize = 0;
  let mut next: usize = 0;
  parse_node(&numbers, &mut next, &mut total, false);
  println!("Total is {}", total);
}

pub fn run2(filename: &String) {
  let input = std::fs::read_to_string(filename).expect("Failed to read input");
  let numbers: Vec<usize> = input.split(" ").map(|x| x.parse().unwrap()).collect();
  let mut total: usize = 0;
  let mut next: usize = 0;
  parse_node(&numbers, &mut next, &mut total, true);
  println!("Total is {}", total);
}
