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
use levenshtein::levenshtein;
use storing::{bk::{TraversalPath}, data::Book, serialize::FileSystemSerializer, library::Library};

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
        library.serialize(path_to_library.clone());
    }
}

fn shelve(input: ShelveCommand, library: &mut Library) -> bool {
    let isbn = match Isbn::from_str(&input.isbn) {
       Ok(result) => result,
       Err(error) => panic!("Encountered problem converting input to ISBN due to: {:?}", error)
    };
    let book = Book { title: input.title, author: input.author, pub_date: input.publish_date, isbn, borrower: None }; 
    println!("Adding book: {}", book.title);
    library.add_node(TreeData::BkBook(book));
    return true;
}

fn search(input: SearchCommand, library: &Library) -> bool {
    let search_result = library.search(input.search_str);
    for r in search_result {
        match r.contents {
            TreeData::BkBook(book) => println!("{} ({})", book.title, r.distance),
            TreeData::BkAuthor(author) => println!("{} ({})", author.name, r.distance)
        };
    }
    return false;
}

fn borrow(input: BorrowCommand, library: &mut Library) -> bool {

    println!("Borrowed book with ISBN: {}", input.isbn);
    return true;
}

fn return_(input: ReturnCommand, library: &mut Library) -> bool {
    println!("Returned book with ISBN: {}", input.isbn);
    return true;
}

fn list_borrows(input: ListBorrowsCommand, library: &Library) -> bool {
    let file_str = read_file(BORROWS_PATH);
    let mut file_lines = file_str.lines();
    let users = file_lines.next().unwrap().split(",");
    let mut borrower = &*input.borrower.to_lowercase();
    let mut shortest_dist = 10;
    for u in users {
        let dist = levenshtein(&input.borrower.to_lowercase(), u);
        if  dist < shortest_dist {
            shortest_dist = dist;
            borrower = u;
        }
    }
    if borrower == input.borrower {
        println!("Could not find user: {}", input.borrower);
        return false;
    }
    let mut result: Vec<&TreeData> = Vec::new();
    for l in file_lines {
        let (b, path) = l.split_once(";").unzip();
        if b.unwrap() == borrower {
            let tp = TraversalPath::deserialize(&path.unwrap());
            result.push(library.get_indices(tp));
        }
    }
    if result.is_empty() {
        println!("{} has not borrowed any books", borrower);
        return false;
    }
    println!("{} has borrowed the following book(s)", borrower);
    for r in result {
        match r {
            TreeData::BkBook(book) => println!("{}", book.title),
            TreeData::BkAuthor(author) => println!("{}", author.name)
        };
    }

    return false;
}


