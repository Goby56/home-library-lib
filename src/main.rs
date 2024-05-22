mod args;
pub mod storing;

use std::{fs::File, io::{Read, Write}, str::FromStr};

use clap::Parser;

use args::{
    Cli, LibraryInteraction,
    ShelveCommand, SearchCommand,
    BorrowCommand, ReturnCommand,
    ListBorrowsCommand
};
use isbn::Isbn;
use levenshtein::levenshtein;
use storing::{bk::{BkTree, TraversalPath}, data::Book, serialize::Serializer};

use crate::storing::data::TreeData;

const LIBRARY_PATH: &str = "data/library.txt";
const BORROWS_PATH: &str = "data/borrows.txt";

fn main() {
    let args = Cli::parse();

    let mut library = BkTree::deserialize(&read_file(LIBRARY_PATH));
    
    let should_save = match args.action {
        LibraryInteraction::Shelve(input) => shelve(input, &mut library),
        LibraryInteraction::Search(input) => search(input, &library),
        LibraryInteraction::Borrow(input) => borrow(input, &mut library),
        LibraryInteraction::Return(input) => return_(input, &mut library),
        LibraryInteraction::ListBorrows(input) => list_borrows(input, &library)
    };

    if should_save {
        write_file(LIBRARY_PATH, library.serialize());
    }
}

fn read_file(path: &str) -> String { 
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}

fn write_file(path: &str, deserialized_str: String) {
    let mut file = File::create(path).unwrap();
    match file.write_all(deserialized_str.as_bytes()) {
       Ok(_) => (),
       Err(error) => panic!("Could not save to disk: {}", error)
    };
}

fn shelve(input: ShelveCommand, library: &mut BkTree) -> bool {
    let isbn = match Isbn::from_str(&input.isbn) {
       Ok(result) => result,
       Err(error) => panic!("Encountered problem converting input to ISBN due to: {:?}", error)
    };
    let book = Book { title: input.title, author: input.author, pub_date: input.publish_date, isbn, borrower: None }; 
    println!("Adding book: {}", book.title);
    library.add_node(TreeData::BkBook(book));
    return true;
}

fn search(input: SearchCommand, library: &BkTree) -> bool {
    let search_result = library.search(input.search_str);
    for r in search_result {
        match r.data {
            TreeData::BkBook(book) => println!("{} ({})", book.title, r.distance),
            TreeData::BkAuthor(author) => println!("{} ({})", author.name, r.distance)
        };
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

fn list_borrows(input: ListBorrowsCommand, library: &BkTree) -> bool {
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
            result.push(library.get_contents(tp));
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


