/*fn insertion_sort(array: &mut [int]) {
  let mut index: int;
  let mut val: int;

  for j in range(1u, array.len()) {
    val = array[j];
    index = j as int - 1;

    while index >= 0 && array[index as uint] > val {
      array[index as uint + 1] = array[index as uint];
      index -= 1;
    };
    array[index as uint + 1] = val;
  };
}*/

fn insertion_sort(array: &mut [int]) {
  for (index, element) in array.iter().enumerate() {
    let mut j = index;
    while j >= 0 && array[j] > *element {
      array[j + 1] = array[j];
      j -= 1;
    };
    array[j + 1] = *element;
  };
}

/*// std::cmp::Ord returns an ordering between `self` and `other` values
// thus, we have a mutable array whose order we can rearrange
fn insertion_sort<T: std::cmp::Ord>(array: &mut [T]) {
  for i in range(1, array.len()) {
    let mut j = i;
    while j > 0 && array[j] < array[j-1] {
      array.swap(j, j-1);
      j = j-1;
    };
  };
}*/

fn main() {
  let mut array = [31i, 41, 59, 26, 41, 58, 60];
  insertion_sort(array);
  for j in range(0u, array.len()) {
    println!("{}", array[j]);
  };
}
