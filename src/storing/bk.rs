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

pub struct BkNode {
    pub identifier: String,
    pub data: BkData,
    pub children: HashMap<u16, BkNode>
}

impl BkTree {
    pub fn init(identifier: String) -> Self {
        BkTree { root: BkNode::from(identifier), bk_paths: Vec::new() }
    }

    pub fn from(root: BkNode) -> Self {
        BkTree { root, bk_paths: Vec::new() }
    }

    pub fn add_node(&mut self, identifier: String) {
        let new_node = BkNode::from(identifier);
        let mut path = TraversalPath::new();
        self.root.add(&mut path, new_node);
        self.bk_paths.push(path);
    }

    pub fn search(&self, query: String) -> Vec<SearchResult> {
        let mut result: Vec<SearchResult> = Vec::new();
        let tolerance = (query.len() as f32 * 0.7).floor().max(1.0) as u16;
        self.root.search(&query.to_lowercase(), tolerance, &mut result);
        return result;
    }
    
    pub fn get_indices(&self, path: TraversalPath) -> &BkData {
        let mut node = &self.root;
        for dist in path.iter() {
            node = node.children.get(dist).unwrap();
        }
        return &node.data;
    }
}

impl BkNode {
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

    pub fn from(identifier: String) -> Self {
        BkNode::create(identifier, Vec::new())
    }

    pub fn create(identifier: String, indices: Vec<u32>) -> Self {
        let (first, id) = identifier.split_at(1);
        match first {
            "@" => BkNode { 
                identifier: id.to_string(), 
                data: BkData::Author(indices), 
                children: HashMap::new() 
            },
            _ => BkNode { 
                identifier, 
                data: BkData::Book(indices), 
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
