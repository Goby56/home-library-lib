use super::bk::BkTree;
use super::data::{Book, Author};

pub struct Library {
    pub bk_tree: BkTree,
    pub flat_books: Vec<Book>
}
