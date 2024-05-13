use std::{collections::HashMap};
use levenshtein::levenshtein;

use super::book::Book;

const MAX_DIST: u16 = 10;

pub struct BkTree {
    pub root: BkNode,
    pub bk_paths: Vec<TraversalPath>
}

pub struct TraversalPath {
    pub path: Vec<u16>
}

impl TraversalPath {
    pub fn new() -> Self {
        return TraversalPath { path: Vec::new() }
    }

    fn append(&mut self, dist: u16) {
        self.path.push(dist);
    }
}

impl BkTree {
    pub fn from(book: Book) -> Self {
        let root = BkNode::from(book);
        return BkTree { root, bk_paths: Vec::new() }
    }

    fn search(&self, query: String) -> Vec<Book> {
        let mut result: Vec<Book> = Vec::new();
        self.root.search(&query, &mut result);
        return result;
    }

    fn add_book(&mut self, book: Book) {
        let new_node = BkNode::from(book);
        let mut path = TraversalPath::new();
        self.root.add(&mut path, new_node);
        self.bk_paths.push(path);
    }
}

pub struct BkNode {
    identifier: String,
    pub(super) book: Book,
    pub(super) children: HashMap<u16, BkNode>
}

impl BkNode {
    pub(super) fn from(book: Book) -> Self {
        return BkNode { identifier: book.title.clone(), book, children: HashMap::new() }
    }

    fn add(&mut self, path: &mut TraversalPath, new_node: BkNode) {
        let dist = self.distance_to(&new_node.identifier);
        path.append(dist);
        match self.child_at(dist) {
            Some(node) => node.add(path, new_node),
            None => {
                self.children.insert(dist, new_node);
            }
        }
    }

    fn distance_to(&self, target: &str) -> u16 {
        return levenshtein(&self.identifier, target).try_into().unwrap();
    }

    fn child_at(&mut self, dist: u16) -> Option<&mut BkNode> {
        return self.children.get_mut(&dist);
    }

    fn search(&self, query: &str, result: &mut Vec<Book>) {
        let dist = self.distance_to(query);

        for (child_dist, node) in &self.children {
            if dist.abs_diff(*child_dist) <= MAX_DIST {
                result.push(self.book.clone());
                node.search(query, result);
            }
        }
    }
}
