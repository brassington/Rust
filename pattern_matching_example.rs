fn main() {
  let array = ["Arrays", "are", "for", "values", "of", "the", "same", "type"];
  let tuple = ("Tuples", 'r', 4i, 0xDEADBEEFi);

  let uno = match array {
    // below is an array pattern, which mirrors the syntax for array literals
    // an underscore in a pattern will ignore a single element
    // a double dot '..' in a pattern will ignore multiple elements
    [_, _, _, values, ..] => values
  };

  // pattern matching can also be employed with declaring variables
  // this will declare two new variables in the current scope, 'dos' and 'tres'
  let (_, dos, _, tres) = tuple;

  println!("{:s} {:c} {:x}!", uno, dos, tres);  // prints "values r deadbeef!"
}
