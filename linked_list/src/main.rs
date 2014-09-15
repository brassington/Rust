// need to allocate List in heap memory using <Box> since
// we don't know how much memory to allocate to it at compile time
pub enum Node {
  // the usage of Cons is not special; it can be called something else 
  Cons(uint, Box<Node>),   // Cons represents the node and its pointer to the next node in the linked list
  Nil
}

impl Node {
  // create a new empty node
  fn new_node() -> Node {
    Nil
  }
  // consumes a list and returns a new list with the specified element appended to the front of the list
  fn append_to_front(self, elem: uint) -> Node {  // we use non-referenced `self` here because we're lending self it out so we can alter it
    Cons(elem, box self)
  }
  // checks if the specified node exists in the list
  fn contains(&self, elem: uint) -> bool {   
    match *self {
      // check each node in the list in a recursive fashion
      Cons(x, ref tail) => if x == elem { true } else { tail.contains(elem) },
      Nil => false
    }
  }
  // if it does, consumes the list and returns a new list with the specified element removed from the list
  // returns the removed node 

  // return the length of the list
  fn length(&self) -> uint {  // we can use a reference here since we aren't altering the linked list
    // `self` has to be matched due to the behavior of this method depends on the variant of `self`
    // `&self` has type `&Node`, `self` has type `Node`
    // matching on a concrete type `T` is preferred over matching on a reference `&T`
    match *self {
      // can't take ownership of the tail, since `self` is a borrowed reference
      // instead, take a reference to the tail
      Cons(_, ref tail) => 1 + tail.length(),
      // an empty list returns a length of 0
      Nil => 0
    }
  }
  // return a representation of the list as a heap allocated string
  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => {
        // `format!` returns a heap allocated string
        format!("{}, [ ] -> {}", head, tail.stringify())
      },
      Nil => {
        format!("Nil")
      }
    }
  }
  // checks to see if the linked list contains a cycle
/*  fn has_cycle(&self) -> bool {
    // since each Node is an owned piece of memory on the heap, it is impossible 
    // to have multiple pointers point to the same Node 
  }*/
}

fn main() {
  let mut list = Node::new_node();
  list = list.append_to_front(1);
  list = list.append_to_front(2);
  list = list.append_to_front(3);
  println!("Linked list contains: {}", list.contains(2));
  println!("Linked list has length: {}", list.length());
  println!("{}", list.stringify());
}
