mod args;
pub mod storing;


use clap::Parser;

use args::{
    Cli, LibraryInteraction,
    ShelveCommand, SearchCommand,
    BorrowCommand, ReturnCommand
};
use storing::{bk::BkTree, book::Book, serialize::Serializer};

fn main() {
    let args = Cli::parse();

    match args.action {
        LibraryInteraction::Shelve(input) => shelve(input),
        LibraryInteraction::Search(input) => search(input),
        LibraryInteraction::Borrow(input) => borrow(input),
        LibraryInteraction::Return(input) => return_(input),
    };

    let books = BkTree::deserialize("storing/books.txt");
}

fn shelve(input: ShelveCommand) {
    
    println!("{}", input.title);
}

fn search(input: SearchCommand) {
    println!("You searched for {}", input.search_str);
}

fn borrow(input: BorrowCommand) {
    println!("Borrowed book with ISBN: {}", input.isbn);
}

fn return_(input: ReturnCommand) {
    println!("Returned book with ISBN: {}", input.isbn);
}


