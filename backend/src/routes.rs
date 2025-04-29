use actix_web::{get, post, web::{self, Data}, Responder, Result,};

use crate::{database, types::{self, Book}, AppState};

#[post("/shelve")]
pub async fn shelve(state: Data<AppState>, data: web::Json<Book>) -> actix_web::Result<String> {
    match database::insert_book(&state.db, data.0.clone()).await {
        Ok(_) => Ok(format!("Added {}", data.title)),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    }
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
