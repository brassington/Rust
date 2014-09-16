// A doubly linked list with owned nodes
use std::util;
use std::cast;
use std::ptr;
use std::iter::Invert;
// This doubly linked list is constructed like a singly linked list
// over the field 'next'.
// Each node owns its own 'next' link, with the last node's next being 'Nil'

// Backlinks DList::prev in this doubly linked list are raw pointers that form 
// a full chain in the reverse direction

pub struct DList<T> {
  list_head: Option<Box<Node<T>>,
  list_tail: Rawlink<Node<T>>
} 

struct Rawlink<T> { p: *mut T }

pub struct Node<T> {
  next: Option<Box<Node<T>>,
  prev: Rawlink<Node<T>>,
  value: T,
}

impl<T> Container for DList<T> {
  fn is_empty(&self) -> bool {
    self.list_head.is_none()
  }
  fn len(&self) -> uint {
    let mut node = &self.list_head;
    let mut i = 0;
    loop {
      match *node {
        Some(ref n) => {
          i += 1;
          node = &n.next;
        }
        None => {
          return i;
        }
      }
    }
  }
}

impl<T> List<T> {
  pub fn new() -> DList<T> {
    DList{ list_head: None, list_tail: Rawlink::none() }
  }
  fn push_front_node(&mut self, mut new_head: Box<Node<T>>) {
    match self.list_head {
      None => {
        self.list_tail = Rawlink::some(new_head);
        new_head.prev = Rawlink::none();
        self.list_head = Some(new_head);
      }
      Some (ref mut head) => {
        new_head.prev = Rawlink::none();
        head.prev = Rawlink::some(new_head);
        util::swap(head, &mut new_head);
        head.next = Some(new_head);
      }
    }
  }
  #[inline]
  pub fn iter<'a>(&'a self) -> DListIterator<'a, T> {
    DListIterator{ nelem: self.len(), head: &self.list_head, tail: self.list_tail }
  }
  pub fn rev_iter<'a>(&'a self) -> Invert<DListIterator<'a, T>> {
    self.iter().invert()
  }
}

impl<T> Node<T> {
  fn new(v: T) -> Node<T> {
    Node{ value: v, next: None, prev: Rawlink::none() }
  }
}

impl<T> Rawlink<T> {
  fn none() -> Rawlink<T> {
    Rawlink{ p: ptr::mut_null() }
  }
  fn some(n: &mut T) -> Rawlink<T> {
    Rawlink{ p: ptr::to_nut_unsafe_ptr(n) }
  }
  fn resolve_immut(&self) -> Option<&T> {
    unsafe { self.p.to_option() }
  }
  fn resolve(&mut self) -> Option<&mut T> {
    if self.p.is_null() {
      None 
    } else {
      Some(unsafe { case::transmute(self.p) })
    }
  }
  fn take(&mut self) -> Rawlink<T> {
    util::replace(self, Rawlink::none())
  }
}

pub struct DListIterator<self, T> {
  head: &self Option<Box<Node<T>>,
  tail: Rawlink<Node<T>>,
  nelem: uint,
}

