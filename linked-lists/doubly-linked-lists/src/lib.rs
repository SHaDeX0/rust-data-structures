use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
struct Node<T: PartialEq + Clone> {
	value: T,
	next: Option<Rc<RefCell<Node<T>>>>,
	previous: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> Node<T>
	where T: PartialEq + Clone {
	pub fn new(value: T) -> Rc<RefCell<Self>> {
		Rc::new(RefCell::new(Node {
			value,
			next: None,
			previous: None,
		}))
	}
}

#[derive(Clone)]
pub struct DoublyLinkedList<T: PartialEq + Clone> {
	head: Option<Rc<RefCell<Node<T>>>>,
	tail: Option<Rc<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T>
	where T: PartialEq + Clone {
	pub fn new(value: T) -> Self {
		let node = Node::new(value);
		DoublyLinkedList {
			head: Some(node.clone()),
			tail: Some(node),
		}
	}

	pub fn insert_at_start(&mut self, value: T) {
		let node = Node::new(value);
		match self.head.take() {
			None => {
				self.head = Some(node.clone());
				self.tail = Some(node);
			}
			Some(head) => {
				head.borrow_mut().previous = Some(node.clone());
				node.borrow_mut().next = Some(head);
				self.head = Some(node);
			}
		}
	}

	pub fn insert_at_end(&mut self, value: T) {
		let node = Node::new(value);
		match self.tail.take() {
			None => {
				self.head = Some(node.clone());
				self.tail = Some(node);
			}
			Some(tail) => {
				tail.borrow_mut().next = Some(node.clone());
				node.borrow_mut().previous = Some(tail);
				self.tail = Some(node);
			}
		}
	}

	pub fn delete_from_start(&mut self) -> Option<T> {
		self.head.take().map(|old_head| {
			match old_head.borrow_mut().next.take() {
				None => {
					self.tail.take();
				}
				Some(new_head) => {
					new_head.borrow_mut().previous.take();
					self.head = Some(new_head);
				}
			}
			Rc::try_unwrap(old_head).ok().unwrap().into_inner().value
		})
	}

	pub fn delete_from_end(&mut self) -> Option<T> {
		self.tail.take().map(|old_tail| {
			match old_tail.borrow_mut().previous.take() {
				None => {
					self.head.take();
				}
				Some(new_tail) => {
					new_tail.borrow_mut().next.take();
					self.tail = Some(new_tail);
				}
			}
			Rc::try_unwrap(old_tail).ok().unwrap().into_inner().value
		})
	}

	pub fn peek_start(&self) -> Option<T> {
		Self::peek(&self.head)
	}

	pub fn peek_end(&self) -> Option<T> {
		Self::peek(&self.tail)
	}

	fn peek(node: &Option<Rc<RefCell<Node<T>>>>) -> Option<T> {
		match node {
			None => None,
			Some(node_rc) => Some(node_rc.borrow().value.clone())
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::DoublyLinkedList;

	#[test]
	fn insert_at_start() {
		let mut list = DoublyLinkedList::new(10);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(10), list.peek_end());

		list.insert_at_start(9);
		assert_eq!(Some(9), list.peek_start());
		assert_eq!(Some(10), list.peek_end());

		list.insert_at_start(8);
		assert_eq!(Some(8), list.peek_start());
		assert_eq!(Some(10), list.peek_end());

		list.insert_at_start(7);
		assert_eq!(Some(7), list.peek_start());
		assert_eq!(Some(10), list.peek_end());
	}

	#[test]
	fn insert_at_end() {
		let mut list = DoublyLinkedList::new(10);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(10), list.peek_end());

		list.insert_at_end(9);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(9), list.peek_end());

		list.insert_at_end(8);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(8), list.peek_end());

		list.insert_at_end(7);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(7), list.peek_end());
	}

	#[test]
	fn delete_at_start() {
		let mut list = DoublyLinkedList::new(10);
		list.insert_at_end(9);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(9), list.peek_end());

		list.delete_from_start();
		assert_eq!(Some(9), list.peek_start());
		assert_eq!(Some(9), list.peek_end());

		list.delete_from_start();
		assert_eq!(None, list.peek_start());
		assert_eq!(None, list.peek_end());

		list.insert_at_start(10);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(10), list.peek_end());
	}

	#[test]
	fn delete_at_end() {
		let mut list = DoublyLinkedList::new(10);
		list.insert_at_end(9);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(9), list.peek_end());

		list.delete_from_end();
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(10), list.peek_end());

		list.delete_from_end();
		assert_eq!(None, list.peek_start());
		assert_eq!(None, list.peek_end());

		list.insert_at_end(10);
		assert_eq!(Some(10), list.peek_start());
		assert_eq!(Some(10), list.peek_end());
	}
}
