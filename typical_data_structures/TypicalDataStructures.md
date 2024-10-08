# Typical Data Structures
--------------------------------------------
## Linked Lists
==================================================
- Linked list is a data structure that consists of a sequence of elements called nodes.
- Each node contains a reference to node next to it.
==================================================
### Singly Linked List
==================================================
- Singly Linked list is a type of linked list where each node points to the next node in the sequence. That is the reference is unidirectional.
```rust
#[derive(Debug)]
struct Node {
    element: i32,
    next: Option<Box<Node>>,
}

pub fn main() {
    let list = Node {
        element: 1,
        next: None,
    };

    println!("{:?}", list);
    println!("element: {}, next: {:?}", list.element, list.next);

    let list = Node {
        element: 1,
        next: Some(Box::new(Node {
            element: 2,
            next: None,
        })),
    };

    println!("{:?}", list);
}
```
- Above code, is one of the way we can implement singly linked list in Rust, but this code has couple of limitations.
- Firstly, this will not allows us to have an explicit information about the head or the starting node.
- Secondly, it will also not allow for an empty list.
- Let's refine the above code to overcome these limitations.
- To have an explicit information regarding the head, we can define another wrapper struct with only one field, called head, which will be of type `Node`.
```rust
#[derive(Debug)]
struct LinkedList {
    head: Node,
}

fn main() {
    // ---------- *snip* ---------- (already defined above)
    let list = LinkedList {
        head: Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 2,
                next: None,
            })),
        },
    };
}
```
- This implementation provides explicit info regarding the head of the list, so the first issue is resolved.
- However, it still doesn't allow for an empty list. To resolve this, we can use an `Option` type for the head field in the `LinkedList` struct.
```rust
#[derive(Debug)]
struct LinkedList {
    head: Option<Node>>,
}
```
- Now, we can have an empty list by setting the head field to `None`.
- Right now, types of head, and next are quite complex, usually it is a good idea to use type aliases to simplify the code. So let's do this.
```rust
struct LinkedList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

fn main() {
    let list = LinkedList {
        head: Some(Box::new(Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            }))
        }))
    };
}
```
- Now, the code is much more readable and understandable.
==================================================
#### Methods on Singly Linked List
==================================================
- Let's implement some methods on the singly linked list.
```rust
impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add(&mut self, element: i32) {
        match self.head {
            None => {
                let new_node = Box::new(Node {
                    element: element,
                    next: None,
                });
                self.head = Some(new_node);
            }
            Some(previous_head) => {
                let new_node = Some(Box::new(Node {
                    element: element,
                    next: Some(previous_head),
                }));
                self.head = new_node;
            }
        }
    }
}
```
- But compiler will give an error `cannot move out of "self.head" as enum variant "Some" which is behind a mutable reference`.
- This is because, value inside `Some` (`previous_head`) captures the unwrapped value of the option we are matching on, furthermore it takes the value by ownership.
- If we take ownership of the head, then the list will have no more access to the head, since the remaining nodes can only be accessed through the head, they will become unusable.
- Let's look at it from another point of view. In some sense, we are trying to rip off the head of a data structure that we don't own. If we rip off the head, then we will have no pointer, which we can use to point to the remaining nodes.
- To fix this, we want to leave something in place of the head for some temporary time while we are updating the head.
- Let's see how we can do this using `take` method.
- Syntax: `fn take<T>(dest: &mut T) -> T`, it takes a mutable reference to a value T, replaces the destination value with a value of T, and returns the original value of T.
```rust
let previous_head = self.head.take();
```
- This will replace the head with default value, which is `None`, and return the previous value of the head.
```rust
fn add(&mut self, element: i32) {
    let previous_head = self.head.take();
    let new_node = Some(Box::new(Node {
        element,
        next: previous_head,
    }));
    self.head = new_node;
}
```
- In this case we will always update the head with the new node, and the next field of the new node will point to the previous head.
- In this case, it will be more appropriate to call `new_node` as `new_head`.
- Let's implement a method to print the list.
```rust
 let mut list = LinkedList::new();
list.add(5);
list.add(7);
list.add(10);
list.add(15);
list.add(20);

println!("Linked List: {:?}", list);
```
- This will print the linked list in reverse order, because we are always adding the new node at the head.
- Let's implement a `remove method` to remove the first element from the list.
```rust
fn remove(&mut self) -> Option<i32> {
    match self.head.take() {
        Some(previous_head) => {
            self.head = previous_head.next;
            Some(previous_head.element)
        },
        None => None,
    }
}
```
- This method will remove the first element from the list, and return the element.
- Let's implement a print method to print the list in order.
```rust
fn print(&self) {
    let mut list_traversal = &self.head;
    while !list_traversal.is_none() {
        println!("{:?}", list_traversal.as_ref().unwrap().element);
        list_traversal = &list_traversal.as_ref().unwrap().next;
    }
}
```
- This method will print the list in order.
==================================================
### Doubly Linked List
==================================================
- Singly linked list allows navigation only in one direction, that is forward. Doubly linked list allows navigation in both directions, that is forward and backward.
- Each node is associated with two pointers, one pointing to the next node, and the other pointing to the previous node.
- There is also one critical observation from programming perspective, each and every is being pointed by more than one pointer.
- This means we need to implement, multiple ownership of the node, which is not possible with simple `Box` type.
- We can use `Rc` type, which is a reference counted smart pointer, which allows multiple ownership of the same data.
- In addition to this, we may require to modify the data by accessing the nodes using either the next or the previous pointer.
- This means we not only want to have multiple ownership of the node, but also want to have mutable access to the node.
- This functionality can only be provided by `RefCell` smart pointer, which is wrapped around by `Rc` smart pointer.
```rust
struct Node {
    element: i32,
    next: Pointer,
    previous: Pointer,
}

struct DoublyLinkedList {
    head: Pointer,
    tail: Pointer,
}

type Pointer = Option<Rc<RefCell<Node>>>;

impl DoublyLinkedList {
    fn new() -> Self {
        DoublyLinkedList {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Rc::new(RefCell::new(Node {
            element,
            next: None,
            previous: None,
        }));

        match self.head.take {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            },
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }
}
```
- In add function, if current head is `None` we don't clone the `new_head` into `self.head`, because we want to keep the ownership of the `new_head` to self.head. Using this we have only two owners `self.tail` and `self.head`, with `self.head` having ownership of the `new_head`.
- Let's implement the method to add to the back of doubly linked list.
```rust
fn add_to_back(&mut self, element: i32) {
    let new_tail = Node::new(element);

    match self.tail.take() {
        Some(old_tail) => {
            old_tail.borrow_mut().next = Some(new_tail.clone());
            new_tail.borrow_mut().previous = Some(old_tail.clone())
        },
        None => {
            self.head = Some(new_tail.clone());
            self.tail = Some(new_tail);

        }
    }
}
```
- Where `Node::new` is a helper function to create a new node.
```rust
impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            previous: None,
        }))
    }
}
```
- Method to remove the first element from the doubly linked list.
```rust
fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
        println!("List is empty so we can't remove anything.");
        return None;
    } else {
        let removed_value = self.head.as_ref().unwrap().borrow().element;
        self.head
            .take()
            .map(|old_head| match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().previous = None;
                    self.head = Some(new_head);
                    self.head.clone()
                }
                None => {
                    self.tail = None;
                    println!("List is empty after removal.");
                    None
                }
            });
        Some(removed_value)
    }
}
```
- Method to print the doubly linked list.
```rust
fn print(&self) {
    let mut traversal = self.head.clone();
    while !traversal.is_none() {
        println!("{}", traversal.as_ref().unwrap().borrow().element);
        traversal = traversal.unwrap().borrow().next.clone();
    }
}
```
--------------------------------------------------
## Reference Cycles
--------------------------------------------------
- Rust is known for being a memory safe language, it employs strong mechanism which provides guarantees that you can't have data races.
- However, it doesn't provide same sort of strong guarantees when it comes to memory leaks.
- Rust attempts to make it difficult, but not impossible to create memory that is never cleaned up.
- Memory leaks can happen when using `Rc` and `RefCell` smart pointers, because they allow for multiple ownership and interior mutability respectively.
- Using these smart pointers, a set of references can be created, where items reference each other in a cycle.
- This will cause a memory leak, because the reference count of each item in the cycle will never reach zero.
- Let's see an example of reference cycle.
```rust

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    next: Option<Rc<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node, {:?}", self);

    }
}

pub fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!("a count: {:?}", Rc::strong_count(&a));

    let b = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&a))}));
    println!("B is created.\na count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node { next: Some(Rc::clone(&b))}));
    println!("C is created.\na count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));
    println!("c count: {:?}", Rc::strong_count(&c));

    // Now let's create a reference cycle
    // c -> b -> a -> c

    (*a).borrow_mut().next = Some(Rc::clone(&c));   // This lines create cycle
    println!("After creating reference cycle.");
    println!("a count: {:?}", Rc::strong_count(&a));
    println!("b count: {:?}", Rc::strong_count(&b));
    println!("c count: {:?}", Rc::strong_count(&c));

    // Let's trigger an overflow
    // println!("a: {:?}", a); // This will trigger a stack overflow

}
```
- In the above code we have created a reference cycle, where `c` points to `b`, `b` points to `a`, and `a` points to `c`.
- In this case, we have also implemented a drop, but it never gets called, because the reference count of each item in the cycle will never reach zero.
- All `a`, `b` and `c` all have reference count of 2, because they are being pointed by two references. So the drop method will never be called.
- If we comment out the code, which creates the reference cycle, then the drop method will be called, and the reference count will be 1.
- Rust provides a nice solution for handling reference cycles, instead of using `Rc` we can use `Weak` smart pointer.
- `Weak` is a special type of `Rc` smart pointer with two key function, `upgrade` and `downgrade`.
- `upgrade` will attempt to convert it into `Rc` thereby increasing the `strong_count` by 1.
- The `downgrade` method creates a new `Weak` pointer to the allocation, the new pointer, will hold a Non-owning reference to the managed allocation, which means it will not provide shared ownership of data. Moreover it increases the weak count by 1 and doesn't change the `strong_count`.
- In summary, the strong references are how you share ownership of a reference counting smart pointer instance. Weak references don't express an ownership relation.
```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    next: Option<Weak<RefCell<Node>>>,
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("Dropping Node, {:?}", self);

    }
}

pub fn main() {
    let a = Rc::new(RefCell::new(Node { next: None }));
    println!("a strong count: {:?}", Rc::strong_count(&a));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    let b = Rc::new(RefCell::new(Node { next: Some(Rc::downgrade(&a))}));
    println!("B is created.\na strong count: {:?}", Rc::strong_count(&a));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    println!("b count: {:?}", Rc::strong_count(&b));

    let c = Rc::new(RefCell::new(Node { next: Some(Rc::downgrade(&b))}));
    println!("C is created.\na strong count: {:?}", Rc::strong_count(&a));
    println!("b strong count: {:?}", Rc::strong_count(&b));
    println!("c strong count: {:?}", Rc::strong_count(&c));
    println!("c weak count: {:?}", Rc::weak_count(&c));
    println!("b weak count: {:?}", Rc::weak_count(&b));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    // Now let's create a reference cycle
    // c -> b -> a -> c

    // Using weak reference to avoid reference cycle

    (*a).borrow_mut().next = Some(Rc::downgrade(&c));   // This lines create cycle
    println!("After creating weak reference cycle.");
    println!("a strong count: {:?}", Rc::strong_count(&a));
    println!("b strong count: {:?}", Rc::strong_count(&b));
    println!("c strong count: {:?}", Rc::strong_count(&c));
    println!("c weak count: {:?}", Rc::weak_count(&c));
    println!("b weak count: {:?}", Rc::weak_count(&b));
    println!("a weak count: {:?}", Rc::weak_count(&a));

    // Now following will not cause stack overflow
    println!("a: {:?}", a); // This will trigger a stack overflow

}
```
- In the above code, we have used `Weak` smart pointer to avoid reference cycle.
- `Weak` pointer doesn't prevent the value it points to from being dropped, and it doesn't increase the strong reference count.
- The next of `a` is not being displayed as it is a weak pointer and by default, the allocation pointed to by weak pointer are not displayed.
- There are some reasons behind not displaying the value of weak pointer,
    - Firstly, they correspond to cycles as explained
    - Furthermore, they don't prevent the value from being dropped, so it is not guaranteed that the value will be there when we try to access it.