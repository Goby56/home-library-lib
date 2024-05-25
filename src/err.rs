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
            Some(title) => write!(f, "{} already borrowed by {}", title, self.borrower.clone().unwrap()),
            None => write!(f, "Book with ISBN {} not found ", self.isbn_search)
        }
    }
}
