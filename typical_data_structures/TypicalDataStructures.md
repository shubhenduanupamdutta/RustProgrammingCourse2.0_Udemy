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