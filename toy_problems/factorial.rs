/* The branches in this function exhibit rust's optional implicit return 
values, which can be utilized where a more 'functional' style is preferred.
Unlike C++ and related languages, rust's 'if' construct is an expression
rather than a statement, and thus it has a return value of its own. 
*/

fn recursive_factorial (n: uint) -> uint {
  if n <= 1 {1}
  else { n * recursive_factorial(n - 1) }
}

fn iterative_factorial (n: uint) -> uint {
  let mut i = 1;
  let mut result = 1;
  while i <= n {
    result *= i;
    i += 1;
  }
  return result;
}

fn main() {
  println!("Recursive function returns: {}", recursive_factorial(10));
  println!("Iterative function returns: {}", iterative_factorial(10));
}
