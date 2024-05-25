use isbn::Isbn;

use crate::err::BookBorrowingError;

use super::bk::BkTree;
use super::data::{Book, Borrows};

pub struct Library {
    search_tree: BkTree,
    books: Vec<Book>,
    borrows: Borrows
}

impl Library {
    pub fn add_book(&mut self, book: Book) {
        self.books.push(book.clone());
        let index = self.books.len() as u32;
        self.search_tree.add_node(book.title, vec![index]);
        self.search_tree.add_node(book.author, vec![index]);
    } 

    pub fn borrow(&mut self, borrower: String, isbn: Isbn) -> Result<(), BookBorrowingError> {
        for i in 0..(self.books.len() - 1) {
            let book = self.books[i];
            if book.isbn != isbn {
                continue;
            }
            match book.borrower {
                Some(b) => Err(BookBorrowingError { book_title: book.title, borrower: b, isbn_search: isbn.to_string() }),
                None => {
                    self.books[i].borrower = Some(borrower);
                    Ok(())
                }
            };
        }
        Err(BookBorrowingError { book_title: None, borrower: None, isbn_search: isbn.to_string() })
    }
}
