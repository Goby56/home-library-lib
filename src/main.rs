// mod args;
// pub mod storing;
// pub mod searching;
// pub mod apis;
// mod err;
mod db;

use std::str::FromStr;

use actix_web::{get, web::Data, App, HttpServer};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

pub struct AppState {
    db: Pool<Sqlite>
}

// #[get("/")]
// async fn index(mut state: Data<AppState>) -> String {
// }
// 
// #[post("/shelve", format = "plain", data = "<isbn>")]
// async fn shelve(mut state: Data<AppState>) -> String {
//     return String::from("Could not find book with ISBN: {isbn}");
// }

// async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
//     match LibraryDB::fetch(&rocket) {
//         Some(db) => match sqlx::migrate!("db/migrations").run(&**db).await {
//             Ok(_) => Ok(rocket),
//             Err(err) => {
//                 error!("Failed to initialize database: {}", err);
//                 Err(rocket)
//             }
//         },
//         None => Err(rocket)
//     }
// }

#[get("/")]
async fn index(state: Data<AppState>) -> String {
    format!("Hello actix web!") // <- response with app_name
}

async fn init_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let db_options = SqliteConnectOptions::from_str("sqlite://db/db.sqlite")?
        .extension("./spellfix1");

    let pool = SqlitePool::connect_with(db_options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    return Ok(pool);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = init_database().await.expect("Could not initialize database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
        .run().await
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

