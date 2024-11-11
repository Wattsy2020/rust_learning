// Lifetimes are also needed when structs hold references.

use std::fmt::{Display, Formatter};

// TODO: Fix the compiler errors about the struct.
struct Book<'a, 'b> {
    author: &'a str,
    title: &'b str,
}

impl<'a, 'b> Display for Book<'a, 'b> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{book}");
    
    // The mixed lifetime lets the borrow checker release that book.author lives beyond this block
    // while book.title doesn't
    let author = "Douglas Adams";
    let author_ref = {
        let title = "Hitchhiker's Guide to the Galaxy".to_string();
        let book = Book { author, title: &title };
        println!("{book}");
        book.author
    };
    println!("{}", author_ref);
}
