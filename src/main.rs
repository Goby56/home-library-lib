mod args;
pub mod storing;

use std::{fs::File, io::{Read, Write}, str::FromStr};

use clap::Parser;

use args::{
    Cli, LibraryInteraction,
    ShelveCommand, SearchCommand,
    BorrowCommand, ReturnCommand
};
use isbn::Isbn;
use storing::{bk::BkTree, book::Book, serialize::Serializer};

const LIBRARY_PATH: &str = "books.txt";

fn main() {
    let args = Cli::parse();

    let mut library = BkTree::deserialize(&read_books_from_disk(LIBRARY_PATH));
    
    let should_save = match args.action {
        LibraryInteraction::Shelve(input) => shelve(input, &mut library),
        LibraryInteraction::Search(input) => search(input, &library),
        LibraryInteraction::Borrow(input) => borrow(input, &mut library),
        LibraryInteraction::Return(input) => return_(input, &mut library),
    };

    println!("{:?}", library);
    
    if should_save {
        write_books_to_disk(LIBRARY_PATH, library.serialize());
    }
}

fn read_books_from_disk(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn write_books_to_disk(path: &str, deserialized_str: String) {
    let mut file = File::create(path).unwrap();
    match file.write_all(deserialized_str.as_bytes()) {
       Ok(_) => (),
       Err(error) => panic!("Can't save books to disk: {}", error)
    };
}

fn shelve(input: ShelveCommand, library: &mut BkTree) -> bool {
    let isbn = match Isbn::from_str(&input.isbn) {
       Ok(result) => result,
       Err(error) => panic!("Encountered problem converting input to ISBN due to: {:?}", error)
    };
    let book = Book { 
        title: input.title, author: input.author, 
        pub_date: input.publish_date, isbn
    };
    library.add_book(book);
    return true;
}

fn search(input: SearchCommand, library: &BkTree) -> bool {
    let result = library.search(input.search_str);
    for book in result {
        println!("{}", book.title);
    }
    return false;
}

fn borrow(input: BorrowCommand, library: &mut BkTree) -> bool {
    println!("Borrowed book with ISBN: {}", input.isbn);
    return true;
}

fn return_(input: ReturnCommand, library: &mut BkTree) -> bool {
    println!("Returned book with ISBN: {}", input.isbn);
    return true;
}


