//---------------------------------------------------------------------------
//           Displaying Participants of an Online Meeting
//              - Description
//                  - Retrieving list of paginated view of the list of participants in an online meeting
//              - Tools
//                 - Binary Search Tree (BST) + Stack
//---------------------------------------------------------------------------
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
struct Node {
    val: String,
    left: Link,
    right: Link,
}

type Link = Option<Rc<RefCell<Node>>>;

impl Node {
    fn new(val: String) -> Self {
        Node {
            val,
            right: None,
            left: None,
        }
    }

    fn insert(&mut self, val: String) {
        if val > self.val {
            match &self.right {
                None => {
                    self.right = Some(Rc::new(RefCell::new(Node::new(val))));
                }
                Some(node) => {
                    node.borrow_mut().insert(val);
                }
            }
        } else {
            match &self.left {
                None => {
                    self.left = Some(Rc::new(RefCell::new(Node::new(val))));
                }
                Some(node) => {
                    node.borrow_mut().insert(val);
                }
            }
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
struct BinarySearchTree {
    root: Node,
}

impl BinarySearchTree {
    fn new(val: String) -> Self {
        BinarySearchTree {
            root: Node::new(val),
        }
    }

    fn insert(&mut self, val: String) {
        self.root.insert(val);
    }
}

struct DisplayLobby {
    stack: Vec<Rc<RefCell<Node>>>,
}

impl DisplayLobby {
    fn new(root: Link) -> Self {
        let mut stack = Vec::new();
        Self::push_all_left(root, &mut stack);
        DisplayLobby { stack }
    }

    fn push_all_left(mut p: Link, stack: &mut Vec<Rc<RefCell<Node>>>) {
        while let Some(link) = p.clone() {
            stack.push(p.clone().unwrap());
            p = link.borrow().left.clone();
        }
    }

    fn next_name(&mut self) -> String {
        let node = self.stack.pop().unwrap();
        let name = &node.borrow().val;
        let next_node = node.borrow().right.clone();
        Self::push_all_left(next_node, &mut self.stack);
        name.to_string()
    }

    fn next_page(&mut self) -> Vec<String> {
        let mut next_names = Vec::new();
        for _ in 0..10 {
            if !self.stack.is_empty() {
                next_names.push(self.next_name());
            } else {
                break;
            }
        }
        next_names
    }
}

pub fn main() {
    let mut bst = BinarySearchTree::new("Jeanette".to_string());
    let names = vec![
        "Latasha",
        "Elvira",
        "Caryl",
        "Antionetter",
        "Cassie",
        "Charity",
        "Lyn",
        "Lia",
        "Anya",
        "Albert",
        "Cherlyn",
        "Lala",
        "Kandince",
        "Iliana",
        "Dutta",
        "Shubhendu",
    ];

    names.into_iter().for_each(|name| {
        bst.insert(name.to_string());
    });

    let mut display = DisplayLobby::new(Some(Rc::new(RefCell::new(bst.root))));
    println!("Participants on the first page: \n{:#?}", display.next_page());

    println!();
    println!("Participants on the second page: \n{:#?}", display.next_page());
}
