//---------------------------------------------------------------------------------
//        Most Recently Used Product
//           - Description
//               - A business is interested in knowing the products that has been purchased recently by a customer.
//           - Tools
//              - HashMaps, Doubly Linked Lists
//---------------------------------------------------------------------------------
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    prod_id: i32,
    prev: Link,
    next: Link,
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            prod_id: element,
            prev: None,
            next: None,
        }))
    }
}

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Default, Debug)]
struct List {
    head: Link,
    tail: Link,
}

impl List {
    fn new() -> Self {
        List {
            head: None,
            tail: None,
        }
    }

    pub fn push_back(&mut self, element: i32) -> Link {
        let new_tail = Node::new(element);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
        self.tail.clone()
    }

    pub fn remove_front(&mut self) -> Option<Link> {
        self.head
            .take()
            .map(|old_head| match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    Some(old_head.clone())
                }
                None => {
                    self.tail.take();
                    None
                }
            })
    }

    pub fn move_to_tail(&mut self, node: &Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.as_ref().map(|a| Rc::clone(a));
        let next = node.borrow().next.as_ref().map(|a| Rc::clone(a));

        match (prev, next) {
            (None, None) => {
                println!("Node in question is the only node in the list. No need to do anything");
            },
            (Some(_), None) => {
                println!("Node in question is at the tail of list. No need to do anything");
            },

            (None, Some(next)) => {
                println!("Node in question is at the head of list");
                println!("Move the head to next node and add the removed node to tail");
                // Node in question is at the head of list
                // First remove the node from the head, and move the head to next node
                node.borrow_mut().next = None;      // Head node's next is None, so now it is separate from the list 
                next.borrow_mut().prev = None;      // Next node's prev is None, so now it is the head of the list
                self.head = Some(next.clone());     // Move the head to next node

                // Update to tail of the list, i.e. removed node addition to tail
                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone())
            },
            (Some(prev), Some(next)) => {
                println!("Node in question is in the middle of the list");
                println!("Remove the node from the list, and add it to the tail");
                // Node in question is in the middle of the list
                // First remove the node from the list, and then add it to the tail
                node.borrow_mut().next = None;       // Node's next is None, so now it is separate from the list

                // Update the prev node's next to next node
                prev.borrow_mut().next = Some(next.clone());
                next.borrow_mut().prev = Some(prev.clone());

                // Update to tail of the list, i.e. removed node addition to tail
                let prev_tail = self.tail.as_ref().unwrap();
                prev_tail.borrow_mut().next = Some(node.clone());
                node.borrow_mut().prev = Some(prev_tail.clone());
                self.tail = Some(node.clone())
            },
        }
    }
}

#[derive(Debug)]
struct MostRecentlyPurchasedItems {
    map: HashMap<i32, Rc<RefCell<Node>>>,
    item_list: List,
    size: i32,
    capacity: i32,
}

impl MostRecentlyPurchasedItems {
    fn new(capacity: i32) -> Self {
        Self {
            map: HashMap::new(),
            item_list: List::new(),
            size: 0,
            capacity,
        }
    }

    fn purchased(&mut self, prod_id: i32) {
        println!();
        if let Some(node) = self.map.get(&prod_id) {
            println!("Product `{}` is already in the list. Move it to the tail", prod_id);
            self.item_list.move_to_tail(node);
        } else {
            println!("Product `{}` is not in the list. Add it to the tail", prod_id);
            if self.size >= self.capacity {
                println!("List is full. Remove the head node and add the new node to the tail");
                let prev_head = self.item_list.remove_front().unwrap();
                println!("Removing product `{}` from the list", prev_head.as_ref().unwrap().borrow().prod_id);
                self.map.remove(&prev_head.unwrap().borrow().prod_id);
            }
            let node = self.item_list.push_back(prod_id).unwrap();
            self.map.insert(prod_id, node);
            self.size += 1;     // We are checking for size >= capacity because we are increasing the size at the end.
        }
    }

    fn print(&self) {
        let mut traversal = self.item_list.head.clone();
        print!("None <- ");
        while !traversal.is_none() {
            let temp = traversal.clone().unwrap();
            print!("|{}| <-> ", temp.borrow().prod_id);
            traversal = temp.borrow().next.clone();
        }
        // Delete the last ` <-> ` and print `None`
        print!("\x08\x08\x08\x08-> "); // Move back 4 characters and overwrite with spaces

        print!("None");
        println!();
    }
}

pub fn main() {
    let mut items_list = MostRecentlyPurchasedItems::new(3);
    items_list.purchased(10);
    print!("After purchasing product `10` \t: ");
    items_list.print();

    items_list.purchased(15);
    print!("After purchasing product `15` \t: ");
    items_list.print();

    items_list.purchased(20);
    print!("After purchasing product `20` \t: ");
    items_list.print();

    items_list.purchased(10);
    print!("After purchasing product `10` \t: ");
    items_list.print();

    items_list.purchased(25);
    print!("After purchasing product `25` \t: ");
    items_list.print();

    items_list.purchased(15);
    print!("After purchasing product `15` \t: ");
    items_list.print();

}
