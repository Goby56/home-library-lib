use std::str::FromStr;

use isbn::Isbn;

use super::bk::{BkTree, BkNode, TraversalPath};
use super::book::Book;

pub trait Serializer {
    fn serialize(&self) -> String;

    fn deserialize(ser_str: String) -> Self;
}

impl Serializer for BkTree {
    fn serialize(&self) -> String{
        let mut deserialized_nodes: Vec<String> = Vec::new();

        for traversal in &self.bk_paths {
            let mut curr_node = &self.root;
            for key in &traversal.path {
                curr_node = curr_node.children.get(&key).unwrap();
            }
            
            let line = format!("{};{}", traversal.serialize(), curr_node.serialize());
            deserialized_nodes.push(line);
        }

        return deserialized_nodes.join("\n");
    }

    fn deserialize(ser_nodes: String) -> Self {

    }
}

impl Serializer for TraversalPath {
    fn serialize(&self) -> String {
        return self.path.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",");
            
    }

    fn deserialize(serialized_path: String) -> Self {
        let path: Vec<u16> = serialized_path
            .split(",")
            .map(|s| s.parse::<u16>().unwrap())
            .collect();
        return TraversalPath { path };
    }
}

impl Serializer for BkNode {
    fn serialize(&self) -> String {
        return self.book.serialize();
    }

    fn deserialize(serialized_book: String) -> Self {
        let book = Book::deserialize(serialized_book);
        return BkNode::from(book);
    }
}

impl Serializer for Book {
    fn serialize(&self) -> String {
       return format!("{},{},{},{}", self.title, self.author, self.pub_date, self.isbn.to_string());
    }

    fn deserialize(serialized_book: String) -> Self {
        let fields: Vec<&str> = serialized_book.split(',').collect();
        return Book { 
            title: fields[0].to_string(), author: fields[1].to_string(), pub_date: fields[2].parse::<u16>().unwrap(), 
            isbn: Isbn::from_str(fields[3]).unwrap() }
    }
}
