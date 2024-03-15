# Singly Linked List

---

![Example of a Singly Linked List](https://media.licdn.com/dms/image/D5612AQGKTTGtlO_SVw/article-inline_image-shrink_1000_1488/0/1697221556971?e=1714608000&v=beta&t=7vY8iWaXgSm3ESPQ1qDhuSsfEAkdkB1dpLzfCVYIZGs "Singly Linked List")

---

## Structure of a Singly Linked List

```rust
pub struct SinglyLinkedList<T: PartialEq> {
	head: Option<Box<Node<T>>>,
}
```

Here, `Node<T>` has the following structure:
```rust
struct Node<T: PartialEq> {
	value: T,
	next: Option<Box<Node<T>>>,
}
```

---

## What is a Singly Linked List?

A singly linked list is a uni-directional linked list; So, you can only traverse it in one direction, i.e., from head node to tail node;

---

## What can it do?

1. `insert_at_start` - Inserts a `value` at the start of the list;
2. `insert_at_end` - Inserts a `value` at the end of the list;
3. `delete_from_start` - Deletes the value at the start of the list if present, else does nothing;
4. `delete_from_end` - Deletes the last element of the list if present, else does nothing;
5. `display` - Display all the elements of the list;
6. `peek` - Peek at the head element value;
7. `size` - Gives the size of the list;
---

## When to use Singly Linked List?

- **Efficient insertions and deletions:** Inserting or deleting elements at the beginning or middle of the list is efficient;
- **Dynamic size:** Can grow or shrink dynamically as elements are added or removed;

---

## When ! to use Singly Linked List?

- **Random Access:** Required to iterate from start to end of the list to find the desired item as the space is allocated on the Heap;

---

## Use cases

- When there is a requirement for storing a list of data that doesn't require being randomly accessed;
- Traversing from a source, with specific steps to follow, to reach a destination;
