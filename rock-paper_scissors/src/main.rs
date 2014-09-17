use std::io;

fn main() {
  println!("How many rounds?");
  let input = io::stdin()
                .read_line()
                .ok()
                .expect("Failed to read input.");
  let input_num: Option<uint> = from_str(input.as_slice().trim());
  let num = match input_num {
    Some(num) => num,
    None      => {
      println!("Please input a number!");
      return;
    }
  };
  println!("{}", rock_paper_scissors(num));
}

fn rock_paper_scissors(rounds: uint) -> Vec<Vec<&'static str>> {
  let plays = ["rock", "paper", "scissors"];
  let mut outcomes: Vec<Vec<&'static str>> = vec![];
  let mut played_so_far: Vec<&'static str> = vec![];
  fn combos(rounds_to_go: uint, outcomes: &mut Vec<Vec<&'static str>>, played_so_far: &mut Vec<&'static str>, plays: [&'static str, ..3]) {
    if rounds_to_go == 0 {
      outcomes.push(played_so_far.clone());
      return;
    }
    for &i in plays.iter() {
      played_so_far.push(i);
      combos(rounds_to_go - 1, outcomes, played_so_far, plays);
      played_so_far.pop();
    }
  }
  combos(rounds, &mut outcomes, &mut played_so_far, plays);
  outcomes
}

#[test]
fn rps_tester() {
  assert!(rock_paper_scissors(3u).len() == 27u);
}

