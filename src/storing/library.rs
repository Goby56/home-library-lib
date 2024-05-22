use super::bk::BkTree;
use super::data::{Book, Author};

pub struct Library {
    pub books: BkTree<Book>,
    pub author: BkTree<Author>,
    pub flat_books: Vec<Book>
}
