// -----------------------------------------------------------------------------
//                 IntoIterator
// -----------------------------------------------------------------------------

struct Book {
    title: String,
    author: String,
    genre: String,
}

struct BookIterator {
    properties: Vec<String>,
}

impl Iterator for BookIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.properties.is_empty() {
            Some(self.properties.remove(0))
        } else {
            None
        }
    }
}

impl IntoIterator for Book {
    type Item = String;
    type IntoIter = BookIterator;

    fn into_iter(self) -> Self::IntoIter {
        BookIterator {
            properties: vec![self.title, self.author, self.genre],
        }
    }
}

pub fn main() {
    let book_1 = Book {
        title: "Harry Potter and the Philosopher's Stone".to_string(),
        author: "J.K. Rowling".to_string(),
        genre: "Fantasy".to_string(),
    };

    let book_iterator = book_1.into_iter();

    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());
    // println!("{:?}", book_iterator.next());

    for property in book_iterator {
        println!("{}", property);
    }
}
