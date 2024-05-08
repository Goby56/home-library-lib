use std::collections::HashMap;

use super::book::Book;

struct BkNode {
    book: Book,
    children: HashMap<u16, BkNode>,

}

trait Tree {
    fn search(&self, query: String) -> Vec<String>;
    fn add(&self, book: Book);
    fn distance(&self, src: String, trg: String) -> u16;
    fn child_at_dist(&self, dist: u16) -> BkNode;
}
