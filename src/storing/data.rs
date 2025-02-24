use std::{fmt::Display, str::FromStr};
use chrono::{DateTime, FixedOffset, Local};
use isbn::Isbn;
use uuid::Uuid;

use crate::{apis::google_books, args::ShelveCommand, err::{BookBorrowingError, ShelveError}};

#[derive(Debug, Clone)]
pub struct Book {
    pub uuid: Uuid,
    pub shelf: Option<String>,
    pub borrower: Option<String>,
    pub borrow_date: Option<DateTime<FixedOffset>>,
    pub isbn: Isbn,
    pub metadata: BookMetadata,
}

impl Book {
    pub fn from(input: ShelveCommand) -> Result<Book, ShelveError> {
        if let Ok(isbn) = Isbn::from_str(&input.isbn) {
            let metadata = BookMetadata::from(isbn.clone())?;
            return Ok(Book { 
                uuid: Uuid::new_v4(),
                shelf: input.shelf,
                borrower: None,
                borrow_date: None,
                isbn,
                metadata
            });
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
                uuid: self.uuid.to_string() 
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
    pub author: String,
    pub pub_date: i16,
    pub genre: Option<String>,
    pub pages: Option<u16>,
    pub language: Option<String>
}

impl BookMetadata {
    pub fn from(isbn: Isbn) -> Result<BookMetadata, ShelveError> {
        match google_books::get_book_metadata(isbn) {
            Ok(_) => println!("worky"),
            Err(err) => println!("{err}")
        }
        Ok(BookMetadata {
            title: String::from("GOT"),
            author: String::from("George"),
            pub_date: 0,
            genre: None,
            pages: None,
            language: None
        })
    }

    pub fn get_search_str(&self) -> String {
        format!("{},{}", self.title, self.author)
    }
}
