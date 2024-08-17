//----------------------------------------------------
//            Singly Linked List
//----------------------------------------------------
#[derive(Debug)]
struct LinkedList {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add(&mut self, element: i32) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            },
            None => None,
        }
    }

    fn peek(&self) -> Option<i32> {
        match &self.head {
            Some(current_head) => Some(current_head.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

pub fn main() {
    println!("###### Nodes in a Singly Linked List ######\n");
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

    println!("############## Singly Linked List ##############\n");
    let list = LinkedList {
        head: Some(Box::new(Node {
            element: 1,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };
    println!("Linked List: {:?}", list);
    println!("Head: {:?}", &list.head);

    println!("############## Methods of Singly Linked List ##############\n");
    let mut list = LinkedList::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    // println!("Linked List: {:?}", list);
    list.print();
    println!("Removed: {:?}", list.remove().unwrap());
    println!("Peek: {:?}", list.peek().unwrap());
    list.print();
}
