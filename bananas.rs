fn main() {
  let b = "bananas";
  for i in range(0, b.len()) {
    for j in range(i, b.len() + 1) {
      let curr = b.slice(i, j);
      println!("{} - {}", b.contains(curr), curr);
    }
  }
}
