use std::str::FromStr;

use isbn::Isbn;

use super::bk::{BkTree, BkNode, TraversalPath};
use super::data::{Book, TreeData, Author};

pub trait Serializer {
    fn serialize(&self) -> String;

    fn deserialize(ser_str: &str) -> Self;
}

impl Serializer for BkTree {
    fn serialize(&self) -> String {
        let mut deserialized_nodes: Vec<String> = Vec::new();
        
        deserialized_nodes.push(format!("{}", self.root.data.serialize()));
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
        let mut tree = BkTree::from(TreeData::deserialize(lines.next().unwrap()));
        for line in lines {
            let (path_str, book_str) = line.split_once(";").unzip();
            // Traversal path
            let tp = TraversalPath::deserialize(path_str.unwrap());
            let node = BkNode::from(TreeData::deserialize(book_str.unwrap()));
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
        return self.data.serialize();
    }

    fn deserialize(serialized_book: &str) -> Self {
        let data = TreeData::deserialize(serialized_book);
        return BkNode::from(data);
    }
}

impl Serializer for TreeData {
    fn serialize(&self) -> String {
        match self {
            TreeData::BkBook(book) => book.serialize(),
            TreeData::BkAuthor(author) => author.name.clone()
        }
    }

    fn deserialize(ser_str: &str) -> Self {
        match ser_str.chars().filter(|c| *c == ',').count() {
            0 => Self::BkAuthor(Author { name: ser_str.to_string(), books: Vec::new() }),
            4 => Self::BkBook(Book::deserialize(ser_str)),
            _ => panic!("Too many arguments was provided")
        }
    }
}

impl Serializer for Book {
    fn serialize(&self) -> String {
        format!(
            "{},{},{},{},{}", self.title, self.author, self.pub_date, 
            self.isbn.to_string(), Book::borrower_as_str(self.borrower.clone())
            )
    }

    fn deserialize(ser_str: &str) -> Self {
        let fields: Vec<&str> = ser_str.split(',').collect();
        Book { 
            title: fields[0].to_string(), author: fields[1].to_string(), pub_date: fields[2].parse::<u16>().unwrap(), 
            isbn: Isbn::from_str(fields[3]).unwrap(), borrower: Book::borrower_as_opt(fields[4])
        }
    }
}
