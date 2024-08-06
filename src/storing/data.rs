use std::{str::FromStr, fmt::Display, collections::HashMap};

use isbn::Isbn;

#[derive(Debug)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pub_date: u16,
    pub isbn: Isbn,
    pub borrower: Option<String>
}

pub struct Borrows(pub HashMap<String, Vec<u32>>);

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

impl Borrows {
    pub fn add_ref(&mut self, user: String, book_ref: u32) {
        match self.0.get_mut(&user) {
            Some(books) => books.push(book_ref),
            None => { self.0.insert(user.clone(), vec![book_ref]); }
        }
    }

    pub fn del_ref(&mut self, user: String, book_ref: u32) {
        if let Some(refs) = self.0.get_mut(&user) {
            if let Some(i) = refs.iter().position(|&r| r == book_ref) {
                refs.swap_remove(i);
            }
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
