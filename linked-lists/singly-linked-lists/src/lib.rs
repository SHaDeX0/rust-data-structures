use std::fmt::Debug;

struct Node<T: PartialEq + Debug> {
	value: T,
	next: Option<Box<Node<T>>>,
}

pub struct SinglyLinkedList<T: PartialEq + Debug> {
	head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T>
	where T: PartialEq + Debug {

	/// Generates a new SinglyLinkedList having `value` data in its head node.
	/// # Example
	/// ```
	/// use singly_linked_lists::SinglyLinkedList;
	/// let singly_linked_list = SinglyLinkedList::new(18);
	/// ```
	/// 
	/// * `value` - value of type `T`
	pub fn new(value: T) -> Self {
		Self {
			head: Some(Box::new(Node {
				value,
				next: None,
			}))
		}
	}

	pub fn insert_at_start(&mut self, value: T) {
		let new_node = Some(
			Box::<Node<T>>::new(
				Node {
					value,
					next: self.head.take(),
				}
			)
		);

		self.head = new_node;
	}

	pub fn delete_from_start(&mut self) {
		self.head.take().map(|node| {
			self.head = node.next;
			node.value
		});
	}

	pub fn insert_at_end(&mut self, value: T) {
		if self.head.is_none() {
			self.head = Some(Box::new(Node {
				value,
				next: None,
			}));
			return;
		}

		let mut temp_node = self.head.as_mut().unwrap();
		while temp_node.next.is_some() {
			temp_node = temp_node.next.as_mut().unwrap()
		}

		temp_node.next = Some(Box::new(Node {
			value,
			next: None,
		}));
	}

	pub fn delete_from_end(&mut self) {
		if self.head.is_none() {
			return;
		}

		let mut temp_node = &mut self.head;
		while temp_node.as_mut().unwrap().next.is_some() {
			temp_node = &mut temp_node.as_mut().unwrap().next;
		}
		*temp_node = None;
	}

	pub fn display(&mut self) {
		if self.head.is_none() {
			return;
		}

		let mut temp_node = &mut self.head;
		while temp_node.as_mut().unwrap().next.is_some() {
			println!("{:?}", temp_node.as_ref().unwrap().value);
			temp_node = &mut temp_node.as_mut().unwrap().next;
		}
	}

	pub fn size(&self) -> usize {
		if self.head.is_none() {
			return 0;
		}

		let mut counter = 0;
		let mut temp = self.head.as_ref();
		while temp.is_some() {
			counter += 1;
			temp = temp.unwrap().next.as_ref();
		}

		counter
	}

	pub fn peek(&self) -> Option<&T> {
		self.head.as_ref().map(|node| {
			&node.value
		})
	}
}

#[cfg(test)]
mod tests {
	use crate::SinglyLinkedList;

	#[test]
	fn it_works() {
		let mut list = SinglyLinkedList::new(10);
		list.insert_at_start(9);
		list.insert_at_start(8);
		list.insert_at_end(11);
		list.insert_at_end(12);
		assert_eq!(list.size(), 5);

		list.display();

		assert_eq!(list.peek(), Some(&8));

		list.delete_from_end();
		assert_eq!(list.size(), 4);


		list.delete_from_start();
		list.delete_from_start();
		list.delete_from_start();
		list.insert_at_start(0);
		list.delete_from_start();
		list.delete_from_start();
		list.insert_at_end(0);
		list.delete_from_end();
		list.delete_from_end();
		list.delete_from_end();
		assert_eq!(list.peek(), None);
		assert_eq!(list.size(), 0);
	}
}