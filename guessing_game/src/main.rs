use std::io;
use std::rand;

fn main() {
  println!("Guess the number!");
  let secret_number = (rand::random::<uint>() % 100u) + 1u;
  // println!("The secret number is: {}", secret_number);
  loop {
    println!("Input your guess.");
    let input = io::stdin()
                  .read_line()
                  .ok()
                  .expect("Failed to read line.");
    // input_num is an option type
    let input_num: Option<uint> = from_str(input.as_slice().trim());   // need trim() to trim off the \n we get when a user enters input
    let num = match input_num {   // use a match to either give us the uint inside the option, or print an error
      Some(num) => num,
      None      => {
        println!("Please input a number!");
        continue;
      }
    };
    println!("You guessed: {}", num);
    match compare(num, secret_number) {
      Less    => println!("Too small!"),
      Greater =>  println!("Too bid!"),
      Equal   =>  { 
        println!("You got it!");
        return; 
      },  
    }
  }
}

fn compare(a: uint, b: uint) -> Ordering {
  if a < b { Less }
  else if a > b { Greater }
  else { Equal }
}

