mod args;
pub mod storing;
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
        LibraryInteraction::ListBorrows(input) => list_borrows(input, &library)
    };

    if should_save {
        match library.serialize(path_to_library.clone()) {
            Ok(()) => {},
            Err(error) => panic!("{error}")
        }
    }
}

fn shelve(input: ShelveCommand, library: &mut Library) -> bool {
    let isbn = match Isbn::from_str(&input.isbn) {
       Ok(result) => result,
       Err(error) => panic!("Encountered problem converting input to ISBN due to: {:?}", error)
    };
    let book = Book { title: input.title, author: input.author, pub_date: input.publish_date, isbn, borrower: None }; 
    println!("Adding book: {}", book.title);
    library.try_add_book(book);
    return true;
}

fn search(input: SearchCommand, library: &Library) -> bool {
    let books = library.search(&input.search_str, "TODO IMPL YEAR EXPR");
    if books.is_empty() {
        println!("Found no books");
    }
    for b in books {
        println!("{b}");
    }
    return false;

}

fn borrow(input: BorrowCommand, library: &mut Library) -> bool {
    match library.borrow(input.borrower, Isbn::from_str(&input.isbn).unwrap()) {
        Ok(book) => println!("{} is now borrowed by {}\n", book.title, book.borrower.unwrap()),
        Err(error) => panic!("{error}")
    }

    println!("Borrowed book with ISBN: {}", input.isbn);

    return true;
}

fn return_(input: ReturnCommand, library: &mut Library) -> bool {
    println!("Returned book with ISBN: {}", input.isbn);
    return true;
}

fn list_borrows(input: ListBorrowsCommand, library: &Library) -> bool {
    if let Ok(books) = library.list_borrows(&input.borrower) {
        for b in books {
            println!("{}", b);
        }
    }
    return false;
}


