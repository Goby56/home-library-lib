use std::str::FromStr;

use isbn::Isbn;

use super::bk::{BkTree, BkNode, TraversalPath};
use super::book::Book;

pub trait Serializer {
    fn serialize(&self) -> String;

    fn deserialize(ser_str: &str) -> Self;
}

impl Serializer for BkTree {
    fn serialize(&self) -> String {
        let mut deserialized_nodes: Vec<String> = Vec::new();
        
        deserialized_nodes.push(format!("{}", self.root.book.serialize()));
        for traversal in &self.bk_paths {
            let mut curr_node = &self.root;
            for key in traversal.iter() {
                curr_node = curr_node.children.get(&key).unwrap();
            }
            
            let line = format!("{};{}", traversal.serialize(), curr_node.serialize());
            deserialized_nodes.push(line);
        }

        return deserialized_nodes.join("\n");
    }

    fn deserialize(ser_nodes: &str) -> Self {
        let mut lines = ser_nodes.lines();
        // First line only includes book
        let mut tree = BkTree::from(Book::deserialize(lines.next().unwrap()));
        for line in lines {
            let (path_str, book_str) = line.split_once(";").unzip();
            // Traversal path
            let tp = TraversalPath::deserialize(path_str.unwrap());
            let node = BkNode::from(Book::deserialize(book_str.unwrap()));
            let mut curr_node = &mut tree.root;
            for dist in tp.all_but_last() {
                curr_node = curr_node.children.get_mut(dist).unwrap();
            }
            curr_node.children.insert(tp.last(), node);
            tree.bk_paths.push(tp);
        }
        return tree;
    }
}

impl Serializer for TraversalPath {
    fn serialize(&self) -> String {
        return self.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
            
    }

    fn deserialize(serialized_path: &str) -> Self {
        let path: Vec<u16> = serialized_path
            .split(",")
            .map(|s| s.parse::<u16>().unwrap())
            .collect();
        return TraversalPath(path);
    }
}

impl Serializer for BkNode {
    fn serialize(&self) -> String {
        return self.book.serialize();
    }

    fn deserialize(serialized_book: &str) -> Self {
        let book = Book::deserialize(serialized_book);
        return BkNode::from(book);
    }
}

impl Serializer for Book {
    fn serialize(&self) -> String {
       return format!("{},{},{},{}", self.title, self.author, self.pub_date, self.isbn.to_string());
    }

    fn deserialize(serialized_book: &str) -> Self {
        let fields: Vec<&str> = serialized_book.split(',').collect();
        return Book { 
            title: fields[0].to_string(), author: fields[1].to_string(), pub_date: fields[2].parse::<u16>().unwrap(), 
            isbn: Isbn::from_str(fields[3]).unwrap() }
    }
}
