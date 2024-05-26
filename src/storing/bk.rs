use std::{collections::HashMap, slice::Iter};
use levenshtein::levenshtein;


pub struct BkTree {
    pub root: BkNode,
    pub bk_paths: Vec<TraversalPath>
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
        BkTree { root, bk_paths: Vec::new() }
    }

    pub fn add_node(&mut self, identifier: String, book_refs: Vec<u32>) {
        let new_node = BkNode::create(identifier, book_refs);
        let mut path = TraversalPath::new();
        let node_added = self.root.add(&mut path, new_node);
        if node_added { self.bk_paths.push(path) }
        
    }

    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        let mut result: Vec<SearchResult> = Vec::new();
        let tolerance = (query.len() as f32 * 0.7).floor().max(1.0) as u16;
        self.root.search(&query.to_lowercase(), tolerance, &mut result);
        return result;
    }
    
    pub fn get_references(&self, path: TraversalPath) -> &BkData {
        let mut node = &self.root;
        for dist in path.iter() {
            node = node.children.get(dist).unwrap();
        }
        return &node.data;
    }
}

impl BkNode {
    fn add(&mut self, path: &mut TraversalPath, new_node: BkNode) -> bool {
        let dist = self.distance_to(&new_node.identifier);
        if dist == 0 {
            self.data.add_refs(new_node.data.get_refs());
            return false;
        }
        path.append(dist);
        match self.child_at(dist) {
            Some(node) => return node.add(path, new_node),
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

#[derive(Clone)]
pub struct TraversalPath(pub Vec<u16>);


impl TraversalPath {
    pub fn new() -> Self {
        return TraversalPath(Vec::new());
    }

    pub fn append(&mut self, dist: u16) {
        self.0.push(dist);
    }

    pub fn length(&self) -> usize {
        return self.0.len();
    }

    pub fn all_but_last(&self) -> &[u16] {
        return &self.0[..self.length() - 1];
    }

    pub fn last(&self) -> u16 {
        return *self.0.last().unwrap();
    }

    pub fn iter(&self) -> Iter<'_, u16> {
        return self.0.iter();
    }
}
