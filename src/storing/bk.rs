use std::{collections::HashMap, slice::Iter, fmt::Debug};
use levenshtein::levenshtein;

#[derive(Debug)]
pub struct BkTree<T> {
    pub root: BkNode<T>,
    pub bk_paths: Vec<TraversalPath>
}

#[derive(Debug, Clone)]
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

impl <T: Clone + Nodeable> BkTree<T> {
    pub fn from(data: T) -> Self {
        return BkTree { root: data.as_node(), bk_paths: Vec::new() }
    }

    pub fn add_node(&mut self, data: T) {
        let new_node = data.as_node();
        let mut path = TraversalPath::new();
        self.root.add(&mut path, new_node);
        self.bk_paths.push(path);
    }

    pub fn search(&self, query: String) -> Vec<SearchResult<T>> {
        let mut result: Vec<SearchResult<T>> = Vec::new();
        let tolerance = (query.len() as f32 * 0.7).floor().max(1.0) as u16;
        self.root.search(&query.to_lowercase(), tolerance, &mut result);
        return result;
    }
    
    pub fn get_contents(&self, path: TraversalPath) -> &T {
        let mut node = &self.root;
        for dist in path.iter() {
            node = node.children.get(dist).unwrap();
        }
        return &node.data;
    }
}

#[derive(Debug)]
pub struct BkNode<T> {
    pub identifier: String,
    pub data: T,
    pub children: HashMap<u16, BkNode<T>>
}


pub trait Nodeable {
    fn as_node(&self) -> BkNode<Self> where Self: Sized;
}

impl <T: Clone> BkNode<T> {
    fn add(&mut self, path: &mut TraversalPath, new_node: BkNode<T>) {
        let dist = self.distance_to(&new_node.identifier);
        path.append(dist);
        match self.child_at(dist) {
            Some(node) => node.add(path, new_node),
            None => {
                self.children.insert(dist, new_node);
            }
        }
    }

    fn search(&self, query: &str, tolerance: u16, result: &mut Vec<SearchResult<T>>) {
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

    fn child_at(&mut self, dist: u16) -> Option<&mut BkNode<T>> {
        return self.children.get_mut(&dist);
    }
}

pub struct SearchResult<T> {
    pub data: T,
    pub distance: u16
}
