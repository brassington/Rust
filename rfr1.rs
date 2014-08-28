fn main() {
  let vec = [1i, 2i, 3i];
  let str_vec = ["hey", "there", "yo"];
  print_vec(vec);
  print_vec(str_vec);
}

/*fn print_vec(vector: &[int]) {
  for i in vector.iter() {
    println!("{:d}", *i)
  }
} 

fn print_vec_str(vector: &[&str]) {
  for i in vector.iter() {
    println!("{:s}", *i)
  }
}*/


// type-generic print_vec function
// this function doesn't compile because our generic type T has no restrictions on what kind of thing it is
// this means that we can't guarantee that we'll get something that has the ability to be displayed

/*fn print_vec<T>(vector: &[T]) {   // <T> designates that we're going to be making this function polymorphic over the type T
  for i in vector.iter() {
    println!("{}", i)   // can't deference the i pointer 
  }
}*/


// this will compile
// we designate type T to have a trait that implements the Show trait
fn print_vec<T std::fmt::Show>(vector: &[T]) {
  for i in vector.iter() {
    println!("{}", i)
  }
}
