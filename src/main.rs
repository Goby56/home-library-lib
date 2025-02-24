mod args;
pub mod storing;
pub mod searching;
pub mod apis;
mod err;

use std::{str::FromStr, path::PathBuf};

use clap::Parser;

use args::{
    Cli, LibraryInteraction,
    ShelveCommand, SearchCommand,
    BorrowCommand, ReturnCommand,
    ListBorrowsCommand
};
use uuid::Uuid;
use storing::{data::Book, library::Library, serialize::{FileSystemSerializer, Serializer}};

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
    if let Ok(book) = Book::from(input) {
        println!("Adding book: {}", book.metadata.title);
        library.add_book(book);
    } else {
        println!("Error adding book")
    }
    return true;
}

fn search(input: SearchCommand, library: &Library) -> bool {
    let search_results = library.search(&input.search_str, input.limit.to_owned(), input.year_expr);
    if search_results.is_empty() {
        println!("Found no books");
    } else {
        for result in search_results {
            println!("{} (score: {})", result.book.metadata.title, result.score)
        }
    }
    return false;
}

fn borrow(input: BorrowCommand, library: &mut Library) -> bool {
    match library.modify_borrow(Some(input.borrower), Uuid::deserialize(&input.uuid)) {
        Ok(book) => println!("{} is now borrowed by {}\n", book.metadata.title, book.borrower.unwrap()),
        Err(error) => {
            println!("Cannot borrow book!\n{error}");
            return false;
        }
    }

    return true;
}

fn return_(input: ReturnCommand, library: &mut Library) -> bool {
    match library.modify_borrow(None, Uuid::deserialize(&input.uuid)) {
        Ok(book) => println!("{} has now been returned\n", book.metadata.title),
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
    return false
}
