extern crate testing;
use testing::add_three_times_four;
// use testing::add_three;

#[test]
fn math_checks_out() {
  let result = add_three_times_four(5i);
  assert_eq!(32i, result);
}

/*#[test]
fn test_add_three() {
  let result = add_three(5i);
  assert_eq!(8i, result);
}*/
