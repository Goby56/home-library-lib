use std::{collections::HashMap, slice::Iter};
use levenshtein::levenshtein;

use super::data::{TreeData};

pub struct BkTree {
    pub root: BkNode,
    pub bk_paths: Vec<TraversalPath>
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

impl BkTree {
    pub fn from(data: TreeData) -> Self {
        return BkTree { root: BkNode::from(data), bk_paths: Vec::new() }
    }

    pub fn add_node(&mut self, data: TreeData) {
        let new_node = BkNode::from(data);
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
    
    pub fn get_contents(&self, path: TraversalPath) -> &TreeData {
        let mut node = &self.root;
        for dist in path.iter() {
            node = node.children.get(dist).unwrap();
        }
        return &node.data;
    }
}

pub struct BkNode {
    pub identifier: String,
    pub data: TreeData,
    pub children: HashMap<u16, BkNode>
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
            result.push(SearchResult { data: self.data.clone(), distance: dist });
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

    pub fn from(data: TreeData) -> Self {
        match data.clone() {
            TreeData::BkBook(book) => {
                BkNode { 
                    identifier: book.title.clone(), 
                    data, children: HashMap::new() 
                }
            },
            TreeData::BkAuthor(author) => {
                BkNode {
                    identifier: author.name.clone(),
                    data, children: HashMap::new()
                }
            }
        }
    }
}

pub struct SearchResult {
    pub data: TreeData,
    pub distance: u16
}
