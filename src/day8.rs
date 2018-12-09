// Parse a node, accumulate the sume of its metadata, and return the remainder of the input
fn parse_node(input: &[usize], next: &mut usize, metadata_accumulator: &mut usize) {
  let num_nodes = input[*next];
  *next += 1;
  let num_metadata: usize = input[*next];
  *next += 1;
  for _ in 0..num_nodes {
    parse_node(input, next, metadata_accumulator);
  }
  for _ in 0..num_metadata {
    *metadata_accumulator += input[*next];
    *next += 1;
  }
}

pub fn run1(filename: &String) {
  let input = std::fs::read_to_string(filename).expect("Failed to read input");
  let numbers: Vec<usize> = input.split(" ").map(|x| x.parse().unwrap()).collect();
  let mut total: usize = 0;
  let mut next: usize = 0;
  parse_node(&numbers, &mut next, &mut total);
  println!("Total is {}", total);
}
