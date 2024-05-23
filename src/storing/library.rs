use super::bk::BkTree;
use super::data::Book;
use super::user::User;

pub struct Library {
    pub search_tree: BkTree,
    pub books: Vec<Book>,
    pub borrows: Vec<User>
}
