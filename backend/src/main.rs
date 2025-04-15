pub mod apis;
mod database;

use std::env;
use std::str::FromStr;

use actix_web::{
    get, post,
    web::{self, Data},
    App, HttpServer,
};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite, SqlitePool};

pub struct AppState {
    db: Pool<Sqlite>,
}

#[derive(serde::Deserialize)]
struct ShelveData {
    isbn: String,
}

#[post("/shelve")]
async fn shelve(state: Data<AppState>, data: web::Json<ShelveData>) -> actix_web::Result<String> {
    let mut result = String::new();
    for book in apis::fetch_book_metadata(&data.isbn).await {
        result.push_str(&book.title);
        match database::insert_book(&state.db, book).await {
            Ok(_) => (),
            Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string())),
        };
    }
    Ok(format!("Added {result}"))
}

#[get("/")]
async fn index(state: Data<AppState>) -> actix_web::Result<String> {
    match database::get_all_books(&state.db).await {
        Ok(books) => Ok(books
            .iter()
            .map(|t| t.0.clone())
            .collect::<Vec<String>>()
            .join("\n")),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    }
}

async fn init_database() -> Result<Pool<Sqlite>, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").unwrap();

    let db_options = SqliteConnectOptions::from_str(&database_url)?.extension("backend/spellfix1");

    let pool = SqlitePool::connect_with(db_options).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;

    return Ok(pool);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap(); // Load .env file

    let pool = init_database()
        .await
        .expect("Could not initialize database");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(index)
            .service(shelve)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
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
