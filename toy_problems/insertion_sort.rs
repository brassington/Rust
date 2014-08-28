fn insertion_sort(array: Vec<int>) -> Vec<int> {
  for i in range(1i, array.len()) {
    let &val = array[i];
    let &hole = i;
    while (hole && val < array[hole - 1]) {
      array[hole] = array[hole - 1];
      hole -= 1;
    }
    array[hole] = val;
  }
  array
}
