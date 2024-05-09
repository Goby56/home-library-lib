mod args;
pub mod storing;


//use std::{fs::File, io::Read};

use args::{
    Cli,
    ShelveCommand, SearchCommand,
    BorrowCommand, ReturnCommand
};
use clap::Parser;

use args::LibraryInteraction;


fn main() {
    let args = Cli::parse();

    match args.action {
        LibraryInteraction::Shelve(input) => shelve(input),
        LibraryInteraction::Search(input) => search(input),
        LibraryInteraction::Borrow(input) => borrow(input),
        LibraryInteraction::Return(input) => return_(input),
    };

   //let book = Book {
    //    title: args.title,
    //    author: args.author,
    //    pub_date: args.publish_date,
    //    isbn: args.isbn.parse(),
    //};
    //println!("{:?}", book);
    // TODO CACHE TITLES? ORDER BY ISBN? FAAAST SEARCH
}

fn shelve(input: ShelveCommand) {
    // let file = File::open("books.txt")?;
    // 
    // let mut contents = String::new();
    // file.read_to_string(&mut contents);

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


