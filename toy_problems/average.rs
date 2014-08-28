/* This program takes in a list of one or more numbers on the command line
and outputs the average value of the input numbers. If any command line argument is 
not a valid number, it outputs a warning about the input and skips it, continuing on.*/
use std::os;   // this module allows you to access command line arguments

fn average(numbers: Vec<String>) -> f64 {   // numbers is a Vec<String>
  let mut sum: f64 = 0.0;
  let mut count: f64 = 0.0;
  for num in numbers.iter().skip(1) {     // want to iterate over each String in Vec<String> and skip over 1 element of iter()
    // num is a single String
    match from_str::<f64>(num.as_slice()) {   // num.as_slice() turns a String into a &str; from_str::<f64> turns &str into an f64
      Some(num) => {
        sum += num;
        count += 1.0;
      },
      None => println!("Bad Input: {}", num)
    }
  }
  sum / count
}

fn main() {
  let numbers = os::args();   // numbers is the same type as what std::os::args() returns
  let avg = average(numbers);

  println!("Average: {}", avg);
}

