#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edtion", book.title, book.year);
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

pub(crate) fn main() {
    // Create an immutable Book named `immutabook`
    let immutabook = Book {
        // string literals have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Goedel, Escher, Bach",
        year: 1979,
    };

    let mut mutabook = immutabook;

    // Immutably borrow a immutable object
    borrow_book(&immutabook);
    // Immutably borrow a mutable object
    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    // new_edition(&mut immutabook); <- ERROR!
}
