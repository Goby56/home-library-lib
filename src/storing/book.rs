use std::str::FromStr;

use isbn::Isbn;


pub struct Book {
    pub title: String,
    pub author: String,
    pub_date: u16,
    isbn: Isbn
}

impl Clone for Book {
    fn clone(&self) -> Self {
        return Book { 
            title: self.title.clone(), 
            author: self.author.clone(), 
            pub_date: self.pub_date.clone(), 
            isbn: Isbn::from_str(&self.isbn.to_string()).unwrap()
        };
    }
}

