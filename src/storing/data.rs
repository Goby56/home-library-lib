use std::{str::FromStr, fmt::Display};

use isbn::Isbn;

use super::bk::TraversalPath;

#[derive(Clone)]
pub enum TreeData {
    BkBook(Book),
    BkAuthor(Author)
}

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pub_date: u16,
    pub isbn: Isbn,
    pub borrower: Option<String>
}

impl Book {
    pub fn borrower_as_str(b: Option<String>) -> String {
        match b {
            Some(borrower) => borrower,
            None => String::from("")
        }
    }

    pub fn borrower_as_opt(b: &str) -> Option<String> {
        match b {
            "" => None,
            _ => Some(b.to_string())
        }
    }
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

pub struct Author {
    pub name: String,
    pub books: Vec<TraversalPath>
}

impl Clone for Author {
    fn clone(&self) -> Self {
        return Author {
            name: self.name.clone(),
            books: self.books.to_vec()
        }
    }
}
