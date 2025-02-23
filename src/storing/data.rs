use std::{fmt::Display, os::linux::fs::MetadataExt, str::FromStr};

use chrono::{DateTime, FixedOffset, Local};
use isbn::Isbn;
use uuid::Uuid;

use crate::{args::ShelveCommand, err::BookBorrowingError};

#[derive(Debug, Clone)]
pub struct Book {
    pub uuid: Uuid,
    pub shelf: String,
    pub title: String,
    pub author: String,
    pub pub_date: i16,
    pub metadata: Option<BookMetadata>,
    pub borrower: Option<String>,
    pub borrow_date: Option<DateTime<FixedOffset>>
}

impl Book {
    pub fn from(input: ShelveCommand) -> Book {
        let mut metadata = None;
        if let Ok(isbn) = Isbn::from_str(&input.isbn) {
            // Fetch metadata
            metadata = Some(BookMetadata::from(isbn));
        };
        return Book { 
            uuid: Uuid::new_v4(),
            shelf: input.shelf,
            title: input.title, 
            author: input.author, 
            pub_date: input.publish_date, 
            borrower: None,
            borrow_date: None,
            metadata
        };
    }

    pub fn borrow(&mut self, user: &str) -> Result<(), BookBorrowingError> {
        match &self.borrower {
            Some(curr_owner) => return Err(BookBorrowingError { // Already borrowed
                book_title: Some(self.title.clone()), 
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
                book_title: Some(self.title.clone()), 
                borrower: None, 
                uuid: self.uuid.to_string()
            })
        }
    }
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Debug)]
pub struct BookMetadata {
    pub isbn: Isbn,
    pub genre: Option<String>,
    pub pages: Option<u16>,
    pub language: Option<String>
}

impl BookMetadata {
    pub fn from(isbn: Isbn) -> BookMetadata {
        BookMetadata {
            isbn,
            genre: None,
            pages: None,
            language: None
        }
    }
}

impl Clone for BookMetadata {
    fn clone(&self) -> Self {
        return BookMetadata {
            isbn: Isbn::from_str(&self.isbn.to_string()).unwrap(),
            genre: self.genre.clone(),
            pages: self.pages.clone(),
            language: self.language.clone(),
        }
    }
    
}
