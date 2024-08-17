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
            }))
        }))
    };
    println!("Linked List: {:?}", list);
    println!("Head: {:?}", &list.head);
}
