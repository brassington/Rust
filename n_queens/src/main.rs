#[cfg(test)] extern crate test;
#[cfg(test)] use test::Bencher;

#[cfg(not(test))]
fn main() {
  for num in range(0i32, 16i32) {
    println!("{}", parallel_n_queens(num));
  }
  for num in range(0i32, 16i32) {
    println!("{}", n_queens(num));
  }
}

fn n_queens(n: i32) -> uint {
  return n_queens_helper((1 << n as uint) -1, 0, 0, 0);
}

fn n_queens_helper(all_ones: i32, l_diag: i32, r_diag: i32, col: i32) -> uint {
  let mut solutions = 0;
  let mut valid_spots = !(l_diag | col | r_diag) & all_ones;
  while valid_spots != 0 {
    let spot = -valid_spots & valid_spots;
    valid_spots = valid_spots ^ spot;
    solutions += n_queens_helper(all_ones, (l_diag | spot) << 1, (col | spot), (r_diag | spot >> 1));
  }
  return solutions + ((col == all_ones) as uint)
}

fn parallel_n_queens(n: i32) -> uint {
  let all_ones = (1 << n as uint) - 1;
  let col = 0;
  let l_diag = 0;
  let r_diag = 0;

  let mut receivers = Vec::new();
  let mut valid_spots = !(l_diag | col | r_diag) & all_ones;
  while valid_spots != 0 {
    let spot = -valid_spots & valid_spots;
    valid_spots = valid_spots ^ spot;
    let (tx, rx) = channel();
    receivers.push(rx);
    spawn(proc() {
      tx.send(n_queens_helper(all_ones, (l_diag | spot) << 1, (col | spot), (r_diag | spot) >> 1));
    });
  }
  let mut results = 0u;
  for receiver in receivers.iter() {
    results += receiver.recv();
  }
  return results + ((col == all_ones) as uint)
}

// unit tests

#[test]
fn test_n_queens() {
  let real = vec!(1, 1, 0, 0, 2, 10, 4, 40, 92u);
  for num in range(0, 9i32) {
    assert!(n_queens(num) == *real.get(num as uint));
  }
}

#[test]
fn test_parallel_n_queens() {
  let real = vec!(1, 1, 0, 0, 2, 10, 4, 40, 92u);
  for num in range(0, 9i32) {
    assert!(parallel_n_queens(num) == *real.get(num as uint));
  }
}

#[bench]
fn bench_n_queens(b: &mut Bencher) {
  b.iter(|| {test::black_box(n_queens(14)); });
}

#[bench]
fn bench_parallel_n_queens(b: &mut Bencher) {
  b.iter(|| { test::black_box(parallel_n_queens(14)); });
}
