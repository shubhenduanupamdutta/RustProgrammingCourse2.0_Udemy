//---------------------------------------------------------------------
//       Fetching Top Products
//          - Description
//              - We are given link lists corresponding to top ranked products in different countries
//                We need to combine all these linked list into one consolidated linked list
//                containing the ranks in descending order
//          - Tools
//              - LinkedList, Iterators, Loops

#[derive(Debug)]
struct LinkedList<T: std::fmt::Debug> {
    head: Pointer<T>,
}

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Pointer<T>,
}

type Pointer<T> = Option<Box<Node<T>>>;

impl<T: std::fmt::Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn add(&mut self, element: T) {
        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<T> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn _peek(&self) -> Option<&T> {
        match &self.head {
            Some(current_head) => Some(&current_head.element),
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        loop {
            match list_traversal {
                Some(current_node) => {
                    print!("{:?} -> ", current_node.element);
                    list_traversal = &list_traversal.as_ref().unwrap().next;
                }
                None => {
                    print!("None");
                    break;
                }
            }
        }
        println!("\n");
    }

    fn reverse(&mut self) {
        if self.head.is_none() || self.head.as_ref().unwrap().next.is_none() {
            return;
        }

        let mut previous = None;
        let mut current = self.head.take(); // Take the head of the linked list
        while current.is_some() {
            let next = current.as_mut().unwrap().next.take(); // Take the next node of the current node
            current.as_mut().unwrap().next = previous.take(); // Reverse the link
            previous = current.take(); // Move the previous pointer to the current node
            current = next; // Move the current pointer to the next node
        }
        self.head = previous.take(); // Set the head of the linked list to the last node
    }
}

pub fn main() {
    let mut list1 = LinkedList::new();
    list1.add(45);
    list1.add(40);
    list1.add(35);
    list1.add(23);
    list1.add(11);

    println!("List 1:");
    list1.print();

    let mut list2 = LinkedList::new();
    list2.add(60);
    list2.add(40);
    println!("List 2:");
    list2.print();

    let mut list3 = LinkedList::new();
    list3.add(85);
    list3.add(20);
    list3.add(15);
    println!("List 3:");
    list3.print();

    let mut consolidated = sort_lists(&mut vec![list1, list2, list3]);
    println!("Consolidated List:");
    consolidated.print();

    println!("Reversing the consolidated list, to get the top products in ascending order");
    consolidated.reverse();
    println!("Consolidated List:");
    consolidated.print();
}

fn sort_lists(lists: &mut Vec<LinkedList<i32>>) -> LinkedList<i32> {
    let mut consolidated_list = LinkedList::new();

    loop {
        let values = lists
            .into_iter()
            .map(|list| list.head.as_ref().unwrap().element)
            .collect::<Vec<i32>>(); // Collect head values of all linked lists in a vector
        
        let min_val = *values.iter().min().unwrap();    // Find the minimum value
        let min_index = values.iter().position(|&x| x == min_val).unwrap(); // Find the index of the minimum value, that is the linked list's index in lists vector

        consolidated_list.add(min_val);     // Add the minimum value to the consolidated list
        lists[min_index].remove();                  // Remove the minimum value from the linked list

        if lists[min_index].head.is_none() {    // If the linked list is empty, remove it from the lists vector
            lists.remove(min_index);
        }

        if lists.is_empty() {   // If all linked lists are empty, break the loop
            break;
        }
    }

    consolidated_list
}
