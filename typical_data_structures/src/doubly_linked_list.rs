//----------------------------------------------------------------
//         Doubly Linked List
//----------------------------------------------------------------

use std::{cell::RefCell, rc::Rc};

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
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().previous = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }
            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn add_to_back(&mut self, element: i32) {
        let new_tail = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().previous = Some(old_tail.clone())
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

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

    fn remove_from_back(&mut self) -> Option<i32> {
        if self.tail.is_none() {
            println!("List is empty so we can't remove anything.");
            return None;
        } else {
            let removed_value = self.tail.as_ref().unwrap().borrow().element;
            self.tail
                .take()
                .map(|old_tail| match old_tail.borrow_mut().previous.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next = None;
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head = None;
                        println!("List is empty after removal.");
                        None
                    }
                });
            Some(removed_value)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            previous: None,
        }))
    }
}

pub fn main() {
    let mut list = DoublyLinkedList::new();

    list.add(30);
    list.add(32);
    list.add(34);
    list.add(36);
    list.add(38);

    println!("Original list:");
    list.print();

    println!("\nAfter removing from front:");
    let first_element = list.remove();
    println!("Removed element: {:?}", first_element);
    list.print();

    println!("\nAfter removing from back:");
    let last_element = list.remove_from_back();
    println!("Removed element: {:?}", last_element);
    list.print();

    println!("\nAfter adding to back:");
    list.add_to_back(40);
    list.print();
}
