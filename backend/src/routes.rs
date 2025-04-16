use actix_web::{get, post, web::{self, Data}, Responder, Result,};

use crate::{apis, database, types, AppState};

#[derive(serde::Deserialize)]
struct ShelveData {
    isbn: String,
}

#[post("/shelve")]
pub async fn shelve(state: Data<AppState>, data: web::Json<ShelveData>) -> actix_web::Result<String> {
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

#[derive(serde::Serialize)]
#[serde(transparent)]
struct BooksResponse {
    books: Vec<types::Book>
}

#[get("/books")]
pub async fn index(state: Data<AppState>) -> Result<impl Responder> {
    match database::get_all_books(&state.db).await {
        Ok(books) => Ok(web::Json(BooksResponse { books })),
        _ => Ok(web::Json(BooksResponse { books: vec![] })),
    }
}
