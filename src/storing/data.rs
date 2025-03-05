use std::{fmt::Display, str::FromStr};
use chrono::{DateTime, FixedOffset, Local, NaiveDate};
use isbn::Isbn;

use crate::{apis, err::{BookBorrowingError, ShelveError}};

#[derive(Debug, Clone)]
pub struct Book {
    pub shelf: Option<String>,
    pub borrower: Option<String>,
    pub borrow_date: Option<DateTime<FixedOffset>>,
    pub isbn: Isbn,
    pub metadata: BookMetadata,
}

impl Book {
    pub async fn from(isbn: String) -> Result<Vec<Book>, ShelveError> {
        let mut books = vec![];
        if let Ok(isbn) = Isbn::from_str(&isbn) {
            for metadata in apis::fetch_book_metadata(&isbn).await {
                books.push(Book { 
                    shelf: None,
                    borrower: None,
                    borrow_date: None,
                    isbn: isbn.clone(),
                    metadata
                });
            }
            return Ok(books);
        };
        Err(ShelveError { isbn: None })
    }

    pub fn get_search_str(&self) -> String {
        format!("{},{}", self.isbn, self.metadata.get_search_str())
    }

    pub fn borrow(&mut self, user: &str) -> Result<(), BookBorrowingError> {
        match &self.borrower {
            Some(curr_owner) => return Err(BookBorrowingError { // Already borrowed
                book_title: Some(self.metadata.title.clone()), 
                borrower: Some(curr_owner.to_string()), 
            }),
            None => { // Not borrowed
                self.borrower = Some(user.to_string());
                let local_time = Local::now();
                self.borrow_date = Some(local_time.with_timezone(local_time.offset()));
                Ok(())
            }
        }
    }

    pub fn return_(&mut self) -> Result<(), BookBorrowingError> {
        match &self.borrower {
            Some(_) => {
                self.borrower = None;
                self.borrow_date = None;
                Ok(())
            },
            None => return Err(BookBorrowingError {
                book_title: Some(self.metadata.title.clone()), 
                borrower: None, 
                uuid: self.uuid.to_string()
            })
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.metadata.title)
    }
}

#[derive(Clone, Debug)]
pub struct BookMetadata {
    pub title: String,
    pub authors: Vec<String>,
    pub pub_date: Option<NaiveDate>,
    pub genres: Vec<String>,
    pub pages: Option<u16>,
    pub language: Option<String>
}

impl BookMetadata {
    pub fn get_search_str(&self) -> String {
        format!("{},{}", self.title, self.authors.join(","))
    }
}
