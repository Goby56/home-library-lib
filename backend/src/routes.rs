use std::io::BufReader;
use image;

use actix_web::{get, post, web::{self, Data}, Responder, Result,};

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use crate::{database, types::{self, Book}, AppState};

#[derive(Debug, MultipartForm)]
struct ShelveForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<Book>,
}

#[post("/shelve")]
pub async fn shelve(state: Data<AppState>, MultipartForm(form): MultipartForm<ShelveForm>) -> actix_web::Result<String> {
    database::insert_book(&state.db, form.json.clone()).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    let reader = BufReader::new(form.file.file.reopen()?);

    let img = image::load(reader, image::ImageFormat::WebP)
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    img.save(format!("./../db/images/book-covers/{}.webp", form.json.isbn))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    Ok(format!("Shelved {}. Access its cover at '/book-cover/{}'", form.json.title, form.json.isbn))
}

#[derive(serde::Serialize)]
#[serde(transparent)]
struct MultipleBooksResponse {
    books: Vec<types::Book>
}

#[derive(serde::Deserialize)]
struct BookSearchQueryParams {
    search_str: Option<String>,
    isbn: Option<String>,
    limit: Option<u32>
}

#[get("/books")]
pub async fn get_books(state: Data<AppState>, query: web::Query<BookSearchQueryParams>) -> Result<impl Responder> {
    match database::get_books(&state.db, query.search_str.clone(), query.isbn.clone(), query.limit).await {
        Ok(books) => Ok(web::Json(MultipleBooksResponse { books })),
        _ => Ok(web::Json(MultipleBooksResponse { books: vec![] })),
    }
}

#[derive(serde::Serialize)]
struct SingleBookResponse {
    book: types::Book,
    copies: Vec<types::PhysicalBook>
}

#[get("/book/{isbn}")]
pub async fn get_book(state: Data<AppState>, path: web::Path<(String,)>) -> Result<impl Responder> {
    match database::get_book_from_isbn(&state.db, path.into_inner().0).await {
        Ok((Some(book), copies)) => Ok(web::Json(SingleBookResponse { book, copies })),
        _ => Err(actix_web::error::ErrorNotFound("Book not found")),
    }
}
