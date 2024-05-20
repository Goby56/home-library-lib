use super::bk::TraversalPath;

pub struct BTree {
    pub root: BNode
}

pub struct BNode {
    pub year: u16,
    pub books: Vec<TraversalPath>,
    pub left: BNode,
    pub right: BNode
}
