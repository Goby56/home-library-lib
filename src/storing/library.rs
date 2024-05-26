use std::str::FromStr;

use isbn::Isbn;
use levenshtein::levenshtein;

use crate::err::{BookBorrowingError, ListBorrowsError};

use super::bk::BkTree;
use super::data::{Book, Borrows};

pub struct Library {
    pub search_tree: BkTree,
    pub books: Vec<Book>,
    pub borrows: Borrows
}

impl Library {
    pub fn try_add_book(&mut self, book: Book) {
        self.books.push(book.clone());
        let index = self.books.len() as u32;
        self.search_tree.add_node(book.title, vec![index]);
        self.search_tree.add_node(book.author, vec![index]);
    } 

    pub fn search(&self, query: &str, _year_expr: &str) -> Vec<&Book> {
        match Isbn::from_str(query) {
            Ok(isbn) => return self.flat_search(isbn),
            _ => {} 
        };
        let mut books = Vec::new();
        for result in self.search_tree.search(query) {
            for book_ref in result.contents.get_refs() {
                let b = self.books.get(book_ref as usize);
                if let Some(b) = b {
                    books.push(b);
                }
            }
        }
        return books;
    }

    pub fn borrow(&mut self, borrower: String, isbn: Isbn) -> Result<Book, BookBorrowingError> {
        for i in 0..(self.books.len() - 1) {
            let book = &self.books[i];
            if book.isbn != isbn {
                continue;
            }
            match book.borrower.clone() {
                Some(b) => return Err(BookBorrowingError { 
                    book_title: Some(book.title.clone()), 
                    borrower: Some(b), 
                    isbn_search: isbn.to_string() 
                }),
                None => {
                    self.books[i].borrower = Some(borrower.clone());
                    return Ok(self.books[i].clone());
                }
            };
        }
        Err(BookBorrowingError { book_title: None, borrower: None, isbn_search: isbn.to_string() })
    }

    pub fn list_borrows(&self, borrower: &str) -> Result<Vec<Book>, ListBorrowsError> {
        let input_b = &borrower.to_lowercase();
        let mut best_b = "";
        let mut shortest_dist = 10;
        for u in self.borrows.0.keys() {
            let dist = levenshtein(input_b, &u.to_lowercase());
            if dist < shortest_dist {
                shortest_dist = dist;
                best_b = u;
            }
        }
        if best_b == input_b {
            return Ok(self.get_books(self.borrows.0.get(input_b)));
        }
        match best_b {
            "" => return Err(ListBorrowsError { 
                input_borrower: input_b.to_string(), 
                found_borrower: None
            }),
            _ => return Err(ListBorrowsError { 
                input_borrower: input_b.to_string(), 
                found_borrower: Some(best_b.to_string())
            })
        };
    }

    fn get_books(&self, book_refs: Option<&Vec<u32>>) -> Vec<Book> {
        let mut books: Vec<Book> = Vec::new();
        if let Some(book_refs) = book_refs {
            for r in book_refs {
                let b = self.books.get(*r as usize);
                if let Some(b) = b {
                    books.push(b.clone());
                }
            }
        }
        return books;
    }


    fn flat_search(&self, isbn: Isbn) -> Vec<&Book> {
        todo!();
    }
}
