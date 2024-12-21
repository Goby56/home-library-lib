mod args;
pub mod storing;
pub mod searching;
mod err;

use std::{str::FromStr, path::PathBuf};

use clap::Parser;

use args::{
    Cli, LibraryInteraction,
    ShelveCommand, SearchCommand,
    BorrowCommand, ReturnCommand,
    ListBorrowsCommand
};
use isbn::Isbn;
use storing::{data::Book, serialize::FileSystemSerializer, library::Library};

const LIBRARY_PATH: &str = "data";

fn main() {
    let args = Cli::parse();
    let path_to_library = PathBuf::from_str(LIBRARY_PATH).unwrap();

    let mut library = Library::deserialize(path_to_library.clone()).unwrap();
    
    let should_save = match args.action {
        LibraryInteraction::Shelve(input) => shelve(input, &mut library),
        LibraryInteraction::Search(input) => search(input, &library),
        LibraryInteraction::Borrow(input) => borrow(input, &mut library),
        LibraryInteraction::Return(input) => return_(input, &mut library),
        LibraryInteraction::ListBorrows(input) => list_borrows(input, &library),
    };

    if should_save {
        match library.serialize(path_to_library.clone()) {
            Ok(()) => {},
            Err(error) => panic!("{error}")
        }
    }
}

fn shelve(input: ShelveCommand, library: &mut Library) -> bool {
    let book = Book { 
        title: input.title, 
        author: input.author, 
        pub_date: input.publish_date, 
        isbn: parse_isbn(&input.isbn), 
        borrower: None 
    }; 
    println!("Adding book: {}", book.title);
    library.try_add_book(book);
    return true;
}

fn search(input: SearchCommand, library: &Library) -> bool {
    let books = library.search(&input.search_str, input.limit.to_owned(), input.year_expr);
    if books.is_empty() {
        println!("Found no books");
    } else {
        for b in books {
            println!("{}", b.title)
        }
    }
    return false;
}

fn borrow(input: BorrowCommand, library: &mut Library) -> bool {
    match library.modify_borrow(Some(input.borrower), parse_isbn(&input.isbn)) {
        Ok(book) => println!("{} with ISBN {} is now borrowed by {}\n", book.title, input.isbn, book.borrower.unwrap()),
        Err(error) => {
            println!("Cannot borrow book!\n{error}");
            return false;
        }
    }

    return true;
}

fn return_(input: ReturnCommand, library: &mut Library) -> bool {
    match library.modify_borrow(None, parse_isbn(&input.isbn)) {
        Ok(book) => println!("{} with ISBN {} has now been returned\n", book.title, input.isbn),
        Err(error) => {
            println!("Cannot return book!\n{error}");
            return false;
        }
    }
    return true;
}

fn list_borrows(input: ListBorrowsCommand, library: &Library) -> bool {
    match library.list_borrows(&input.borrower) {
        Ok(books) => books.iter().enumerate().for_each(|(i, b)| println!("{}: {b}", i+1)),
        Err(error) => println!("{error}")
    }
    return false;
}

fn parse_isbn(isbn: &str) -> Isbn {
    return match Isbn::from_str(isbn) {
       Ok(result) => result,
       Err(error) => panic!("Encountered problem converting input to ISBN due to: {:?}", error)
    };
}
