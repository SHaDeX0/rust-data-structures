# Doubly Linked List

---

![Example of a Doubly Linked List](https://media.geeksforgeeks.org/wp-content/cdn-uploads/gq/2014/03/DLL1.png "Doubly Linked List")

---

## Structure of a Doubly Linked List

```rust
pub struct DoublyLinkedList<T: PartialEq + Clone> {
	head: Option<Rc<RefCell<Node<T>>>>,
	tail: Option<Rc<RefCell<Node<T>>>>,
}
```
Here, `Node<T>` has the following structure:
```rust
struct Node<T: PartialEq + Clone> {
	value: T,
	next: Option<Rc<RefCell<Node<T>>>>,
	previous: Option<Rc<RefCell<Node<T>>>>,
}
```

---

## What is a Doubly Linked List?

A doubly linked list is a bidirectional-linked list. So, you can traverse it in both the directions, i.e., from head node to tail node and vice-versa.

---

## What can it do?

1. `insert_at_head` - Inserts a `value` at the head of the list;
2. `insert_at_tail` - Inserts a `value` at the tail of the list;
3. `delete_from_head` - Deletes a node at the head of the list if present, else does nothing;
4. `delete_from_tail` - Deletes a node at the tail of the list if present, else does nothing;
5. `peek_head` - Peek at the head element value;
6. `peek_tail` - Peek at the tail element value;

---

## When to use Doubly Linked List?

- **Bidirectional traversal:** It enables us to traverse in both the directions
- **Efficient Insertion & Deletion:** It has O(1) time complexity at the ***ends*** of the list

---

## When ! to use Doubly Linked List?

- **Random Access:** Required to iterate from start to end of the list to find the desired item as the space is allocated on the Heap.

---

## Use cases

- Implement a Undo & Redo functionality
- Caching - Most Recently Used / Least Recently Used