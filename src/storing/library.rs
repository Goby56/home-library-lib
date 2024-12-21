use std::str::FromStr;

use isbn::Isbn;
use levenshtein::levenshtein;

use crate::err::{BookBorrowingError, ListBorrowsError};
use crate::searching::comparator::Comparison;

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
        let index = (self.books.len() - 1) as u32;
        self.search_tree.add_node(book.title, vec![index]);
        self.search_tree.add_node(format!("@{}", book.author), vec![index]);
    } 

    pub fn search(&self, query: &str, _limit: Option<usize>, year_expr: Option<String>) -> Vec<&Book>  {
        let mut books = Vec::new();
        for result in self.search_tree.search(query) {
            for book_ref in result.contents.get_refs() {
                let b = self.books.get(book_ref as usize);
                if let Some(b) = b {
                    books.push(b);
                }
            }
        }
        if let Ok(isbn) = Isbn::from_str(query) {
            for b in &self.books {
                if b.isbn == isbn {
                    books.push(&b);
                }
            }
        }
        let comp = Comparison::new(">=", 1990).unwrap().compare(query.parse::<i32>().unwrap());
        println!("{comp}");
        // let req = VersionReq::parse(&year_expr.unwrap_or("".to_string()))?;
        return books;
        // return Ok(books.into_iter()
        //     .filter(|b| req.matches(
        //             &Version::parse(&b.pub_date.to_string()).unwrap()))
        //     .collect());
    }

    pub fn modify_borrow(&mut self, user: Option<String>, isbn: Isbn) -> Result<Book, BookBorrowingError> {
        for i in 0..(self.books.len()) {
            let book = &self.books[i];
            if book.isbn != isbn {
                continue;
            }
            match &user {
                Some(new_owner) => match &book.borrower {
                    Some(curr_owner) => return Err(BookBorrowingError { 
                        book_title: Some(book.title.clone()), 
                        borrower: Some(curr_owner.to_string()), 
                        isbn_search: isbn.to_string() 
                    }),
                    None => self.borrows.add_ref(new_owner.to_string(), i as u32)
                },
                None => match &book.borrower {
                    Some(curr_owner) => self.borrows.del_ref(curr_owner.to_string(), i as u32),
                    None => return Err(BookBorrowingError { 
                        book_title: Some(book.title.clone()), 
                        borrower: None, 
                        isbn_search: isbn.to_string() 
                    })
                }
            }
            self.books[i].borrower = user.clone();
            return Ok(self.books[i].clone())
        }
        Err(BookBorrowingError { book_title: None, borrower: None, isbn_search: isbn.to_string() })
    }

    pub fn list_borrows(&self, borrower: &str) -> Result<Vec<Book>, ListBorrowsError> {
        let lc_input = &borrower.to_lowercase();
        let mut best_match = "";
        let mut shortest_dist = 10;
        for user in self.borrows.0.keys() {
            let dist = levenshtein(lc_input, &user.to_lowercase());
            if dist < shortest_dist {
                shortest_dist = dist;
                best_match = user;
            }
        }
        if &best_match.to_lowercase() == lc_input {
            return Ok(self.get_books(self.borrows.0.get(best_match)));
        }
        match best_match {
            "" => return Err(ListBorrowsError { 
                input_borrower: borrower.to_string(), 
                found_borrower: None
            }),
            _ => return Err(ListBorrowsError { 
                input_borrower: borrower.to_string(), 
                found_borrower: Some(best_match.to_string())
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


    // fn flat_search(&self, isbn: Isbn) -> Vec<&Book> {
    //     todo!();
    // }
}
