use std::fmt::Show;

// instantiating each tree node as an enum
pub enum BinaryTree<T> {
  Leaf(T),
  Branch(T, Box<BinaryTree<T>>, Box<BinaryTree<T>>),
  Nil,
}

// creates an immutable binary search tree that can't be altered after created
fn create_binary_search_tree(vector: Vec<int>) -> BinaryTree<int> {
  fn insert_node<T: Copy + Ord + Show> (val: T, btree: BinaryTree<T>) -> BinaryTree<T> {
    match btree {
      Leaf(tval) if val > tval => Branch(tval, box Nil, box Leaf(val)),
      Leaf(tval) if val < tval => Branch(tval, box Leaf(val), box Nil),
      Branch(tval, left, right) => match val.cmp(&tval) {
        Greater => Branch(tval, left, box insert_node(val, *right)),
        Less    => Branch(tval, box insert_node(val, *left), right),
        Equal   => fail!("There already exists a node with value {}", tval),
      },
      Nil => Leaf(val),
      Leaf(lval) if val == lval => Leaf(val),
      _   => Nil,
    }
  }
  let mut tree = Nil;
  for v in vector.iter() {
    tree = insert_node(*v, tree)
  }
  let immutable_tree = tree;
  immutable_tree
}

// prints the tree
fn print_binary_search_tree(tree: &BinaryTree<int>) {
  fn inner_print(prefix: &str, tree: &BinaryTree<int>, level: int) {
    let level_descent = format!("level {}", level);
    match tree {
      &Leaf(val) => println!("{} - {} leaf: {}", level_descent, prefix, val),
      &Branch(val, box ref left, box ref right) => {
        println!("{} - {} node: {}", level_descent, prefix, val);
        inner_print("left branch <-", left, level + 1);
        inner_print("right branch ->", right, level + 1);
      },
      &Nil => println!("end"),
    }
  }
  inner_print("root", tree, 0);
}

fn main() {
    print_binary_search_tree(&create_binary_search_tree(vec![43, 2, 45, 7, 72, 28, 34, 33]))
}
