use std::collections::HashMap;
use levenshtein::levenshtein;


pub struct BkTree {
    pub root: BkNode,
}

#[derive(Clone)]
pub enum BkData {
    Book(Vec<u32>),
    Author(Vec<u32>)
}

impl BkData {
    pub fn add_refs(&mut self, mut book_ref: Vec<u32>) {
        match self {
            BkData::Book(d) | BkData::Author(d) => d.append(&mut book_ref)
        };
    }

    pub fn get_refs(&self) -> Vec<u32> {
        match self {
            BkData::Book(d) | BkData::Author(d) => d.to_vec()
        }
    
    }
}

pub struct BkNode {
    pub identifier: String,
    pub data: BkData,
    pub children: HashMap<u16, BkNode>
}

impl BkTree {
    pub fn init(root: BkNode) -> Self {
        BkTree { root }
    }

    pub fn add_node(&mut self, identifier: String, book_refs: Vec<u32>) {
        let new_node = BkNode::create(identifier, book_refs);
        let node_added = self.root.add(new_node);
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut result: Vec<SearchResult> = Vec::new();
        let tolerance = BkTree::per_query_tolerance(query);
        self.root.search(&query.to_lowercase(), tolerance, &mut result);
        return result;
    }

    fn per_query_tolerance(query: &str) -> u16 {
        (query.len() as f32).powf(0.5).max(1.0) as u16
    }
}

impl BkNode {
    fn add(&mut self, new_node: BkNode) -> bool {
        let dist = self.distance_to(&new_node.identifier);
        if dist == 0 {
            self.data.add_refs(new_node.data.get_refs());
            return false;
        }
        match self.child_at(dist) {
            Some(node) => return node.add(new_node),
            None => {
                self.children.insert(dist, new_node);
                return true;
            }
        };
    }

    fn search(&self, query: &str, tolerance: u16, result: &mut Vec<SearchResult>) {
        let dist = self.distance_to(query);
        
        if dist <= tolerance {
            result.push(SearchResult { contents: self.data.clone(), distance: dist });
        }
        for (child_dist, node) in &self.children {
            let diff = dist.abs_diff(*child_dist);
            if diff <= tolerance {
                node.search(query, tolerance, result);
            }
        }
    }

    fn distance_to(&self, target: &str) -> u16 {
        return levenshtein(&self.identifier.to_lowercase(), target).try_into().unwrap();
    }

    fn child_at(&mut self, dist: u16) -> Option<&mut BkNode> {
        return self.children.get_mut(&dist);
    }

    pub fn create(identifier: String, book_refs: Vec<u32>) -> Self {
        let (first, id) = identifier.split_at(1);
        match first {
            "@" => BkNode { 
                identifier: id.to_string(), 
                data: BkData::Author(book_refs), 
                children: HashMap::new() 
            },
            _ => BkNode { 
                identifier, 
                data: BkData::Book(book_refs), 
                children: HashMap::new() }
        }

    }
}

pub struct SearchResult {
    pub contents: BkData,
    pub distance: u16
}
