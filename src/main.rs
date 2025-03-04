mod args;
pub mod storing;
pub mod searching;
pub mod apis;
mod err;

use std::{path::PathBuf, str::FromStr};

#[macro_use] extern crate rocket;
use rocket::{http::RawStr, State};
use rocket_db_pools::{Database, Connection};
use rocket_db_pools::sqlx;

use storing::{data::Book, library::Library, serialize::FileSystemSerializer};

const LIBRARY_PATH: &str = "data";

#[derive(Database)]
#[database("hllite")]
struct LibraryDB(sqlx::SqlitePool);

#[get("/")]
fn index(state: &State<Library>) -> String {
    let mut content = String::new();
    for b in &state.books {
        content.push_str(&b.metadata.title);
    }
    return content;
}

#[post("/shelve", format = "plain", data = "<isbn>")]
async fn shelve(mut db: Connection<LibraryDB>, isbn: &RawStr) -> String {
    let mut content = String::new();
    if let Ok(books) = Book::from(isbn.to_string()).await {
        for book in &books {
            content.push_str(&book.get_search_str());
        }
        return content;
    }
    return String::from("Could not find book with ISBN: {isbn}");
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let path_to_library = PathBuf::from_str(LIBRARY_PATH).unwrap();

    let _rocket = rocket::build()
        .manage(Library::deserialize(path_to_library.clone()).unwrap())
        .mount("/", routes![index, shelve])
        .launch()
        .await?;

    Ok(())
}

// fn shelve(input: ShelveCommand, library: &mut Library) -> bool {
//     if let Ok(book) = Book::from(input) {
//         println!("Adding book: {}", book.metadata.title);
//         library.add_book(book);
//     } else {
//         println!("Error adding book")
//     }
//     return true;
// }
// 
// fn search(input: SearchCommand, library: &Library) -> bool {
//     let search_results = library.search(&input.search_str, input.limit.to_owned(), input.year_expr);
//     if search_results.is_empty() {
//         println!("Found no books");
//     } else {
//         for result in search_results {
//             println!("{} (score: {})", result.book.metadata.title, result.score)
//         }
//     }
//     return false;
// }
// 
// fn borrow(input: BorrowCommand, library: &mut Library) -> bool {
//     match library.modify_borrow(Some(input.borrower), Uuid::deserialize(&input.uuid)) {
//         Ok(book) => println!("{} is now borrowed by {}\n", book.metadata.title, book.borrower.unwrap()),
//         Err(error) => {
//             println!("Cannot borrow book!\n{error}");
//             return false;
//         }
//     }
// 
//     return true;
// }
// 
// fn return_(input: ReturnCommand, library: &mut Library) -> bool {
//     match library.modify_borrow(None, Uuid::deserialize(&input.uuid)) {
//         Ok(book) => println!("{} has now been returned\n", book.metadata.title),
//         Err(error) => {
//             println!("Cannot return book!\n{error}");
//             return false;
//         }
//     }
//     return true;
// }
// 
// fn list_borrows(input: ListBorrowsCommand, library: &Library) -> bool {
//     match library.list_borrows(&input.borrower) {
//         Ok(books) => books.iter().enumerate().for_each(|(i, b)| println!("{}: {b}", i+1)),
//         Err(error) => println!("{error}")
//     }
//     return false
// }

