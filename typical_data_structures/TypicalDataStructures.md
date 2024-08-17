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