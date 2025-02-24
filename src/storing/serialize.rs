use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use std::str::FromStr;

use chrono::{DateTime, FixedOffset};
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
    let file_result = File::open(path);
    let mut contents = String::new();
    if let Ok(mut file) = file_result {
        file.read_to_string(&mut contents)?;
    }
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
            "{},{},{},{},{},{}\n", 
            self.uuid.to_string(),
            self.shelf.serialize(),
            self.borrower.serialize(),
            self.borrow_date.serialize(),
            self.isbn.serialize(),
            self.metadata.serialize()
            )
    }

    fn deserialize(ser_str: &str) -> Self {
        let fields: Vec<&str> = ser_str.split(',').collect();
        Book { 
            uuid: Uuid::deserialize(fields[0]),
            shelf: Option::deserialize(fields[1]),
            borrower: Option::deserialize(fields[2]),
            borrow_date: Option::deserialize(fields[3]),
            isbn: Isbn::deserialize(fields[4]),
            metadata: BookMetadata::deserialize(fields[5])
        }
    }
}

impl Serializer for BookMetadata {
    fn serialize(&self) -> String {
        format!(
            "{};{};{};{};{};{}\n", 
            self.title,
            self.author,
            self.pub_date.to_string(),
            self.genre.serialize(),
            self.pages.serialize(),
            self.language.serialize()
            )
    }

    fn deserialize(ser_str: &str) -> Self {
        let fields: Vec<&str> = ser_str.split(';').collect();
        BookMetadata { 
            title: fields[0].to_string(),
            author: fields[1].to_string(),
            pub_date: fields[2].parse::<i16>().unwrap(),
            genre: Some(fields[3].to_string()),
            pages: fields[4].parse::<u16>().ok(),
            language: Some(fields[5].to_string()),
        }
        
    }
}
impl Serializer for DateTime<FixedOffset> {
    fn serialize(&self) -> String {
        self.format("%d-%m-%Y %H:%M:%S %z").to_string()
    }

    fn deserialize(ser_str: &str) -> Self {
        // Parses datetimes in the format 23-02-2025 09:59:18 +0100
        DateTime::parse_from_str(ser_str, "%d-%m-%Y %H:%M:%S %z").unwrap()
        
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
