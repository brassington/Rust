/* This program defines a recursive data structure and implements methods upon itself.
Recursive data structures require a layer of indirection, which is provided here 
by a unique pointer, constructed via the 'box' operator. These are analogous to 
the C++ library type 'std::unique_ptr', though with more static safety guarantees. */

fn main() {
  let list = box Node(1, box Node(2, box Node(3, box Empty)));
  println!("Sum of all values in the list: {:i}.", list.multiply_by(2).sum());
}

// 'enum' defines a tagged union that may be one of several different kinds of values at runtime
// the type here will either contain no value, or a value and a pointer to another 'IntList'

enum IntList {
  Node(int, Box<IntList>),  
  Empty
}

// an 'impl' block allows methods to be defined on a type 
impl IntList {
  fn sum(self) -> int {
    match self {
      Node(value, next) => value + next.sum(),
      Empty => 0
    }
  }

  fn multiply_by(self, n: int) -> Box<IntList> {
    match self {
      Node(value, next) => box Node(value * n, next.multiply_by(n)),
      Empty => box Empty
    }
  }
}
