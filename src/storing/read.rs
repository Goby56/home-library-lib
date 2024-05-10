//use std::{fs::File, io::Read};
//
//use super::book::Book;
//use super::bk::BkNode;
//
//fn load_tree(path: String) -> BkNode {
//    let file = File::open(path);
//    
//    let mut contents = String::new();
//    file.read_to_string(&mut contents);
//}
//
//trait Serializable {
//    fn serialize(path: String, root: BkNode);
//
//    fn deserialize(path: String) -> BkNode;
//}
//
//impl Serializable for BkNode {
//    fn serialize(path: String, root: BkNode) {
//        
//    }
//}
