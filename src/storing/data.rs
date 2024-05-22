use std::{str::FromStr, fmt::Display, collections::HashMap};

use isbn::Isbn;

use super::bk::{TraversalPath, Nodeable, BkNode};


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

impl Nodeable for Book {
    fn as_node(&self) -> BkNode<Self> where Self: Sized {
        return BkNode { identifier: self.title.clone(), data: self.clone(), children: HashMap::new() };
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

impl Nodeable for Author {
    fn as_node(&self) -> BkNode<Self> where Self: Sized {
        return BkNode { identifier: self.name.clone(), data: self.clone(), children: HashMap::new() };
    }
}
