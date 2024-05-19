use std::{str::FromStr, fmt::Display};

use isbn::Isbn;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pub_date: u16,
    pub isbn: Isbn,
    pub borrower: Option<String>
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

impl Clone for Book {
    fn clone(&self) -> Self {
        return Book { 
            title: self.title.clone(), 
            author: self.author.clone(), 
            pub_date: self.pub_date.clone(), 
            isbn: Isbn::from_str(&self.isbn.to_string()).unwrap(),
            borrower: self.borrower.clone()
        };
    }
}

