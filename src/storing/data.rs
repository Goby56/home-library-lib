use std::{str::FromStr, fmt::Display};

use isbn::Isbn;
use uuid::Uuid;

use crate::args::ShelveCommand;

#[derive(Debug, Clone)]
pub struct Book {
    pub uuid: Uuid,
    pub shelf: String,
    pub title: String,
    pub author: String,
    pub pub_date: u16,
    pub metadata: Option<BookMetadata>,
    pub borrower: Option<String>,
    pub borrow_date: Option<String>
}

impl Book {
    pub fn from(input: ShelveCommand) {
        let metadata = None;
        if let Ok(isbn) = Isbn::from_str(&input.isbn) {
            // Fetch metadata
        };
        let book = Book { 
            uuid: Uuid::new_v4(),
            shelf: input.shelf,
            title: input.title, 
            author: input.author, 
            pub_date: input.publish_date, 
            metadata, 
            borrower: None,
            borrow_date: None
        }; 
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
