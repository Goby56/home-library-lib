use std::collections::HashMap;

use super::{book::Book, bk::{BkTree, TraversalPath}};

pub struct Library {
    pub bk_tree: BkTree,
    pub authors: HashMap<String, Vec<TraversalPath>>,
    pub b_tree:  
}
