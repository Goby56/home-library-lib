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

#[derive(serde::Deserialize)]
struct BookSearchQueryParams {
    search_str: Option<String>,
    isbn: Option<String>,
    limit: Option<u32>
}

#[get("/get_books")]
pub async fn index(state: Data<AppState>, query: web::Query<BookSearchQueryParams>) -> Result<impl Responder> {
    match database::query_books(&state.db, query.search_str.clone(), query.isbn.clone(), query.limit).await {
        Ok(books) => Ok(web::Json(BooksResponse { books })),
        _ => Ok(web::Json(BooksResponse { books: vec![] })),
    }
}
