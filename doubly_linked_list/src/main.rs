// A doubly linked list with owned nodes

// This doubly linked list is constructed like a singly linked list
// over the field 'next'.
// Each node owns its own 'next' link, with the last node's next being 'Nil'

// Backlinks DList::prev in this doubly linked list are raw pointers that form 
// a full chain in the reverse direction

pub struct DList<T> {
  length: uint,
  list_head: Link<T>,
  list_tail: RawLink<Node<T>>
} 

type Link<T> = Option<Box<Node<T>>>;
struct RawLink<T> { p: *mut T }

struct Node<T> {
  next: Link<T>,
  prev: RawLink<Node<T>>,
  value: T,
}


