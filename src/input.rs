pub fn read_lines(filename: String) -> Vec<String> {
  let rows = std::fs::read_to_string(filename).expect("Failed to read input");

  return rows.split("\n").map(|s| s.to_string()).collect();
}
