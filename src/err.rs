use std::{error::Error, fmt};

#[derive(Clone, Debug)]
pub struct BookBorrowingError {
    pub book_title: Option<String>,
    pub borrower: Option<String>,
    pub isbn_search: String
}

impl Error for BookBorrowingError {}

impl fmt::Display for BookBorrowingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.book_title {
            Some(title) => match &self.borrower {
                Some(borrower) => write!(f, "{} is already borrowed by {}", title, borrower),
                None => write!(f, "{} is not borrowed by anyone", title)
            }
            None => write!(f, "Book with ISBN {} not found ", self.isbn_search)
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
