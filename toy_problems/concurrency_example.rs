// This function creates ten tasks that all execute concurrently.

fn main() {
  // this string is immutable, so it can safely be accessed from multiple tasks
  let greeting = "Hello";
  // for loops work with any type that implements the 'Iterator' trait
  for num in range(0i, 10) {
    spawn(proc() {    // what are spawn and proc?
      println!("{:s} from lightweight thread number {:i}", greeting, num);
    });
  }
}
