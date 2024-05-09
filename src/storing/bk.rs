use std::collections::HashMap;
use levenshtein::levenshtein;

use super::book::{Book, Searchable};

const MAX_DIST: u16 = 10;

struct BkNode {
    book: Book,
    // TODO Memory pooling for nodes for flat memory allocation
    // and linear retrieval
    children: HashMap<u16, BkNode>
}

trait Tree {
    fn search<'a>(&'a self, query: &str, result: &mut Vec<&'a Book>);
    fn add(&mut self, book: Book);
    fn distance_to(&self, target: &str) -> u16;
    fn child_at(&mut self, dist: u16) -> Option<&mut BkNode>;
}

impl Tree for BkNode {
    fn add(&mut self, book: Book) {
        let dist = self.distance_to(&book.search_str());
        match self.child_at(dist) {
            Some(node) => node.add(book),
            None => {
                let node = BkNode { book, children: HashMap::new() };
                self.children.insert(dist, node);
            }
        }
    }

    fn distance_to(&self, target: &str) -> u16 {
        return levenshtein(&self.book.search_str(), target).try_into().unwrap();
    }

    fn child_at(&mut self, dist: u16) -> Option<&mut BkNode> {
        return self.children.get_mut(&dist);
    }

    fn search<'a>(&'a self, query: &str, result: &mut Vec<&'a Book>) {
        let dist = self.distance_to(query);

        for (child_dist, node) in &self.children {
            if dist.abs_diff(*child_dist) <= dist {
                result.push(&self.book);
                node.search(query, result);
            }
        }
    }
}
