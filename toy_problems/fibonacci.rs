fn main() {
  let max = 25;
  let results = fibonacci(max, []);  // pass in max as arg1 and an empty, dynamically-sized vector as arg2
  println!("{:?}", results);    // print the results as a string using '{:?}'
}

// @param uint - max - an unsigned integer, the number of Fibonacci numbers to generate
// @param vector - list - a mutable, empty, dynamically sized vector of integers
// @return ~[int] - we're going to return a list of integers

fn fibonacci(max: uint, mut list: [int]) -> [int] {
  if list.len() == 0 {
    list = [0, 1];   // if the list is empty, give it 0 and 1 as first params to items
  } else if list.len() == max {
    return list;
  }
  // get the last two items in the list
  let n1 = list.len() - 1;   // last item in the list
  let n2 = list.len() - 2;   // second-to-last item in the list
  let f1 = *list.iter().nth(n1).unwrap();   // get the (n-1)th item in the list
  let f2 = *list.iter().nth(n2).unwrap();   // get the (n-2)th item in the list

  // add them together and push the sum into the list using list.push(f1 + f2)
  // get the next number with the new list
  return fibonacci(max, list)
}
