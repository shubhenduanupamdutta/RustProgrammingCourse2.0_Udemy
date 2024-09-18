//---------------------------------------------------------------------------------
//       Items In Range
//          - Description
//              - Give a vector of product prices, search for item in given price range
//          - Tools
//               - Binary Search Tree, Box Pointer

#[derive(Clone)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, val: i32) {
        if val > self.value {
            match self.right {
                None => self.right = Some(Box::new(Node::new(val))),
                Some(ref mut node) => node.insert(val),
            }
        } else {
            match self.left {
                None => self.left = Some(Box::new(Node::new(val))),
                Some(ref mut node) => node.insert(val),
            }
        }
    }
}

struct BinarySearchTree {
    root: Node,
}

fn traversal(node: Option<Box<Node>>, low: i32, high: i32, mut output: &mut Vec<i32>) {
    if node.is_some() {
        // Calling with as_ref() to avoid moving the value out of the Option
        if node.as_ref().unwrap().value >= low && node.as_ref().unwrap().value <= high {
            output.push(node.as_ref().unwrap().value);
        }
        if node.as_ref().unwrap().value >= low {
            traversal(node.as_ref().unwrap().left.clone(), low, high, &mut output);
        }
        if node.as_ref().unwrap().value <= high {
            traversal(node.as_ref().unwrap().right.clone(), low, high, &mut output);
        }
    }
}

fn products_in_range(root: Node, low: i32, high: i32) -> Vec<i32> {
    let mut output = Vec::new();
    traversal(Some(Box::new(root)), low, high, &mut output);
    output
}

pub fn main() {
    let product_prices = vec![9, 6, 14, 20, 1, 30, 8, 17, 5];

    let mut bst = BinarySearchTree {
        root: Node::new(product_prices[0]),
    };

    for i in 1..product_prices.len() {
        bst.root.insert(product_prices[i]);
    }

    let result = products_in_range(bst.root, 7, 20);
    println!("Products in range {:?}", result);
}
