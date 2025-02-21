use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use isbn::Isbn;
use uuid::Uuid;

use super::data::{Book, BookMetadata};
use super::library::Library;


pub trait Serializer {
    fn serialize(&self) -> String;
    fn deserialize(ser_str: &str) -> Self;
}

pub trait FileSystemSerializer {
    fn serialize(&self, path: PathBuf) -> Result<(), Error>;
    fn deserialize(path: PathBuf) -> Result<Self, Error> where Self: Sized;
}

pub trait VecSerializer<T> {
    fn serialize(&self) -> String;
    fn deserialize(ser_vec: &str) -> Self;
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
        self.books.serialize(path.join("books.txt"))?;
        Ok(())
    }

    fn deserialize(path: PathBuf) -> Result<Self, Error> {
        let books = Vec::<Book>::deserialize(path.join("books.txt"))?;
        Ok(Library::from(books))
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
        let contents = read_file(path)?;
        let books = contents.lines().map(|line| Book::deserialize(line)).collect();
        Ok(books)
    }
}

impl Serializer for Book {
    fn serialize(&self) -> String {
        format!(
            "{},{},{},{},{},{},{},{}\n", 
            self.uuid.to_string(),
            self.shelf,
            self.title, 
            self.author, 
            self.pub_date,
            self.borrower.serialize(),
            self.borrow_date.serialize(),
            self.metadata.serialize()
            )
    }

    fn deserialize(ser_str: &str) -> Self {
        let fields: Vec<&str> = ser_str.split(',').collect();
        Book { 
            uuid: Uuid::deserialize(fields[0]),
            shelf: fields[1].to_string(),
            title: fields[2].to_string(), 
            author: fields[3].to_string(), 
            pub_date: fields[4].parse::<i16>().unwrap(), 
            metadata: Some(BookMetadata::deserialize(fields[5])),
            borrower: Option::deserialize(fields[6]),
            borrow_date: Option::deserialize(fields[7])
        }
    }
}

impl Serializer for BookMetadata {
    fn serialize(&self) -> String {
        format!(
            "{},{},{},{}\n", 
            self.isbn.serialize(),
            self.genre.serialize(),
            self.pages.serialize(),
            self.language.serialize()
            )
    }

    fn deserialize(ser_str: &str) -> Self {
        let fields: Vec<&str> = ser_str.split(',').collect();
        BookMetadata { 
            isbn: Isbn::deserialize(fields[0]),
            genre: Some(fields[1].to_string()),
            pages: Some(fields[2].parse::<u16>().unwrap()),
            language: Some(fields[3].to_string()),
        }
        
    }
}

impl Serializer for Isbn {
    fn serialize(&self) -> String {
       self.to_string() 
    }

    fn deserialize(ser_str: &str) -> Self {
        Isbn::from_str(ser_str).unwrap()
    }
}

impl Serializer for Uuid {
    fn serialize(&self) -> String {
        self.to_string()
    }

    fn deserialize(ser_str: &str) -> Self {
        Uuid::parse_str(ser_str).unwrap()
    }
}

impl <T: Serializer> Serializer for Option<T> {
    fn serialize(&self) -> String {
        match self {
            Some(s) => T::serialize(s),
            None => String::from("")
        }
    }

    fn deserialize(ser_str: &str) -> Self {
        match ser_str {
            "" => None,
            _ => Some(T::deserialize(ser_str))
        }
    }
}

impl Serializer for String {
    fn serialize(&self) -> String {
        self.clone()
    }

    fn deserialize(ser_str: &str) -> Self {
        ser_str.to_string()
    }
}

// TODO One impl for all numbers
impl Serializer for i16 {
    fn serialize(&self) -> String {
        self.to_string()    
    }

    fn deserialize(ser_str: &str) -> Self {
       ser_str.parse::<i16>().unwrap() 
    }
}

impl Serializer for u16 {
    fn serialize(&self) -> String {
        self.to_string()    
    }

    fn deserialize(ser_str: &str) -> Self {
       ser_str.parse::<u16>().unwrap() 
    }
}
