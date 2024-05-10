use std::collections::HashMap;
use levenshtein::levenshtein;

use super::book::Book;

const MAX_DIST: u16 = 10;

pub struct BkTree<'bk> {
    children_pool: Vec<BkNode<'bk>>
}

impl<'bk> BkTree<'bk> {
    pub fn new() -> Self {
        return BkTree { children_pool: Vec::new() }
    }

    fn get_mut_root(&mut self) -> &mut BkNode<'bk> {
        return &mut self.children_pool[0];
    }
    
    fn get_root(&self) -> &BkNode<'bk> {
        return &self.children_pool[0];
    }

    fn search(&self, query: String) -> Vec<Book> {
        let mut result: Vec<Book> = Vec::new();
        self.get_root().search(&query, &mut result);
        return result;
    }

    fn add_book(&mut self, book: Book) {
        self.children_pool.push(BkNode { identifier: book.title.clone(), book, children: HashMap::new() });
        self.get_mut_root().add(self.children_pool.last().unwrap());
    }
}

struct BkNode<'bk> {
    identifier: String,
    book: Book,
    children: HashMap<u16, &'bk BkNode<'bk>>
}

impl<'bk> BkNode<'bk> {
    fn add(&mut self, new_node: &'bk BkNode<'bk>) {
        let dist = self.distance_to(&new_node.identifier);
        match self.child_at(dist) {
            Some(mut node) => node.add(new_node),
            None => {
                self.children.insert(dist, new_node);
            }
        }
    }

    fn distance_to(&self, target: &str) -> u16 {
        return levenshtein(&self.identifier, target).try_into().unwrap();
    }

    fn child_at(&mut self, dist: u16) -> Option<&mut &'bk BkNode<'bk>> {
        return self.children.get_mut(&dist);
    }

    fn search(&self, query: &str, result: &mut Vec<Book>) {
        let dist = self.distance_to(query);

        for (child_dist, &node) in &self.children {
            if dist.abs_diff(*child_dist) <= MAX_DIST {
                result.push(self.book.clone());
                node.search(query, result);
            }
        }
    }
}
