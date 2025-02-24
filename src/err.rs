use std::{error::Error, fmt::{self}};

use isbn::Isbn;
#[derive(Clone, Debug)]
pub struct ShelveError {
    pub isbn: Option<Isbn>
}

impl Error for ShelveError {}

impl fmt::Display for ShelveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.isbn {
            Some(isbn) => write!(f, "Could not fetch book metadata from ISBN: {isbn}"),
            None => write!(f, "Invalid ISBN")
        }
    }
}

#[derive(Clone, Debug)]
pub struct BookBorrowingError {
    pub book_title: Option<String>,
    pub borrower: Option<String>,
    pub uuid: String
}

impl Error for BookBorrowingError {}

impl fmt::Display for BookBorrowingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.book_title {
            Some(title) => match &self.borrower {
                Some(borrower) => write!(f, "{} is already borrowed by {}", title, borrower),
                None => write!(f, "{} is not borrowed by anyone", title)
            }
            None => write!(f, "Book with UUID {} not found ", self.uuid)
        }
    }
}

#[derive(Clone, Debug)]
pub struct ListBorrowsError {
    pub input_borrower: String,
    pub found_borrower: Option<String>
}

impl Error for ListBorrowsError {}

impl fmt::Display for ListBorrowsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.found_borrower {
            Some(borrower) => write!(f, "{} not found. Did you mean {borrower}?", self.input_borrower),
            None => write!(f, "Could not find user {}", self.input_borrower)
        }
    }
}


