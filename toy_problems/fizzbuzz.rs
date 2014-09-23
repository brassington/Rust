// A simpler version, no helper methods required
fn main() {

    fn fizz_buzz(n: int) {
        for i in range(1i, n) {
            match i {
                i if (i % 15 == 0) => { println!("FizzBuzz") }
                i if (i % 3 == 0) => { println!("Fizz") }
                i if (i % 5 == 0) => { println!("Buzz") }
                _ => { println!("{:d}", i)}
            }
        }
    }

    fizz_buzz(100);
}

// a more compact implementation
// fn main() {
//   for num in range(1i, 101) {
//     println!("{:s}",
//       if div_by_fifteen(num) { "FizzBuzz".to_string() }
//       else if div_by_three(num) { "Fizz".to_string() }
//       else if div_by_five(num) { "Buzz".to_string() }
//       else { num.to_string() }
//     );
//   }
// }

/* // a less compact implementation
fn main() {  
  for num in range(1i, 101) {
    let answer = 
      if div_by_fifteen(num) {
        "Fizzbuzz".to_string()
      }
      else if div_by_five(num) {
        "Buzz".to_string()
      }
      else if div_by_three(num) {
        "Fizz".to_string()
      }
      else {
        num.to_string()
      };

    println!("{}", answer);
  }
}
*/

fn div_by_three(num: int) -> bool {
  num % 3 == 0 
}

fn div_by_five(num: int) -> bool {
  num % 5 == 0
}

fn div_by_fifteen(num: int) -> bool {
  num % 15 == 0
}

#[test]
fn test_div_by_three() {
  if div_by_three(1) {
    fail!("One is not three");
  }
}

#[test]
fn test_div_by_three_with_three() {
  assert!(div_by_three(3))
}

#[test]
fn test_div_by_five() {
  if div_by_five(1) {
    fail!("One is not five");
  }
}

#[test]
fn test_div_by_five_with_five() {
  assert!(div_by_five(5))
}

#[test]
fn test_div_by_fifteen() {
  if div_by_fifteen(1) {
    fail!("One is not fifteen");
  }
}

#[test]
fn test_div_by_fifteen_with_fifteen() {
  assert!(div_by_fifteen(15))
}
