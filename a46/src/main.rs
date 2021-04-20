#[derive(Debug, Clone)]
struct Book<'a> {
    author: &'a str,
    title: &'a str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I borrowed {} {}", book.author, book.year)
}

fn new_edition(book: &mut Book) {
    book.year = 2018
}

fn main() {
    let mut book = Book {
        author: "a",
        title: "b",
        year: 2000,
    };
    borrow_book(&book);
    new_edition(&mut book);
    borrow_book(&book);
}
