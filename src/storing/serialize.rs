use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use isbn::Isbn;

use super::bk::{BkTree, BkNode, TraversalPath, BkData};
use super::data::Book;
use super::library::Library;
use super::user::User;


pub trait Serializer<T> {
    fn serialize(&self) -> String;

    fn deserialize(ser_str: &str) -> Self;
}

pub trait FileSystemSerializer {
    fn serialize(&self, path: PathBuf) -> Result<(), Error>;

    fn deserialize(path: PathBuf) -> Result<Self, Error> where Self: Sized;
}

fn write_file(path: PathBuf, contents: String) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}

fn read_file(path: PathBuf) -> Result<String, Error> { 
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

impl FileSystemSerializer for Library {
    fn serialize(&self, path: PathBuf) -> Result<(), Error> {
        self.search_tree.serialize(path.join("tree.txt"))?;
        Ok(())
    }

    fn deserialize(path: PathBuf) -> Result<Self, Error> {
        let search_tree = BkTree::deserialize(path)?;
        let books = Vec::<Book>::deserialize(path)?;
        let borrows = Vec::<User>::deserialize(path)?;
        Ok(Library { search_tree, books, borrows })
    }
}

impl FileSystemSerializer for BkTree {
    fn serialize(&self, path: PathBuf) -> Result<(), Error> {
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

        write_file(path, deserialized_nodes.join("\n"))
    }

    fn deserialize(path: PathBuf) -> Result<Self, Error> {
        let mut lines = match read_file(path) {
            Ok(contents) => contents.lines(),
            Err(error) => panic!("{}", error)
        };
        // First line doesn't have path
        let mut tree: BkTree = BkTree::from(BkNode::deserialize(lines.next().unwrap()));
        for line in lines {
            let (path_str, id_and_refs) = line.split_once(";").unzip();
            // Traversal path
            let tp = TraversalPath::deserialize(path_str.unwrap());
            let node = BkNode::deserialize(id_and_refs.unwrap());
            let mut curr_node = &mut tree.root;
            for dist in tp.all_but_last() {
                curr_node = curr_node.children.get_mut(dist).unwrap();
            }
            curr_node.children.insert(tp.last(), node);
            tree.bk_paths.push(tp);
        }
        return Ok(tree);
    }
}

impl FileSystemSerializer for Vec<Book> {
    fn serialize(&self, path: PathBuf) -> Result<(), Error> {
        let serialized_books = self.iter()
            .map(|book| book.serialize())
            .reduce(|mut tot, b| {
                tot.push_str(&b);
                return tot;
            });
        write_file(path, serialized_books.unwrap())
    }

    fn deserialize(path: PathBuf) -> Result<Self, Error> {
        let mut lines = match read_file(path) {
            Ok(contents) => contents.lines(),
            Err(error) => panic!("{}", error)
        };
        let books = lines.map(|line| Book::deserialize(line)).collect();
        Ok(books)
    }
}

impl FileSystemSerializer for User {
    fn serialize(&self, path: PathBuf) -> Result<(), Error> {
        
    }

    fn deserialize(path: PathBuf) -> Result<Self, Error> where Self: Sized {
        
    }
}

impl <T> Serializer<T> for TraversalPath {
    fn serialize(&self) -> String {
        self.0.serialize()
    }

    fn deserialize(ser_path: &str) -> Self {
        TraversalPath(Vec::<u16>::deserialize(ser_path))
    }
}

impl <T> Serializer<T> for BkNode {
    fn serialize(&self) -> String {
        let id = match self.data {
            BkData::Book(_) => self.identifier,
            BkData::Author(_) => format!("@{}", self.identifier)
        };
        format!("{};{}", id, self.data.serialize())
    }

    fn deserialize(ser_str: &str) -> Self {
        let (identifier, book_refs) = ser_str.split_once(";").unzip();
        return BkNode::create(identifier.unwrap().to_string(), Vec::<u32>::deserialize(book_refs.unwrap()));
    }
}

impl <T> Serializer<T> for BkData {
    fn serialize(&self) -> String {
       match self {
           BkData::Book(book_ref) => book_ref.serialize(),
           BkData::Author(book_refs) => book_refs.serialize()
       } 
    }

    fn deserialize(_ser_str: &str) -> Self {
        unimplemented!();
    }
}

impl <T> Serializer<T> for Book {
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

impl <T: ToString + FromStr> Serializer<T> for Vec<T> {
    fn serialize(&self) -> String {
        self.iter()
            .map(|x| x.to_string())
            .reduce(|mut tot, s| {
                tot.push_str(&s);
                return tot;
            }).unwrap()
    }

    fn deserialize(ser_str: &str) -> Self {
        ser_str.split(",")
            .map(|s| -> T {
                let r = s.parse::<T>();
                match r {
                    Ok(t) => t,
                    Err(_) => panic!("Problem reading to vec as {} couldn't be parsed to\n", s)
                }
            })
            .collect()
    }
}

