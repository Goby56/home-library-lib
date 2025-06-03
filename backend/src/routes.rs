use std::io::BufReader;
use image::{self, ImageReader};

use actix_web::{get, post, web::{self, Data}, Responder, Result,};

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use serde::Deserialize;
use crate::{database, types, AppState};

#[derive(Debug, MultipartForm)]
struct ShelveForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<types::Book>,
}

#[post("/register_book")]
pub async fn register_book(state: Data<AppState>, MultipartForm(form): MultipartForm<ShelveForm>) -> actix_web::Result<String> {
    let _book_id = database::insert_book(&state.db, form.json.clone()).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    let reader = BufReader::new(form.file.file.reopen()?);
    let img = ImageReader::new(reader).with_guessed_format()?.decode()
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    img.save(format!("./backend/db/images/book-covers/{}.webp", form.json.isbn))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    Ok(format!("Shelved {}. Access its cover at '/book-cover/{}.webp'", form.json.title, form.json.isbn))
}

#[derive(Deserialize)]
struct ShelfInfo {
    isbn: String,
    name: String
}

#[post("/add_physical_book")]
pub async fn add_physical_book(state: Data<AppState>, shelf_data: web::Json<ShelfInfo>) -> actix_web::Result<String> {
    let shelf = database::get_or_create_shelf(&state.db, shelf_data.name.clone()).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    let book = database::get_books(&state.db, None, Some(shelf_data.isbn.clone()), Some(1), true).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?.pop();
    return match (book, shelf) {
        (Some(book), Some(shelf)) => {
            let _ = database::create_physical_book(&state.db, book.id, shelf.id).await;
            Ok(format!("Added a physical copy of {} to shelf {}", book.title, shelf.name))
        },
        _ => Err(actix_web::error::ErrorNotFound("Couldn't find book or create shelf")),
    };
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
    limit: Option<u32>,
    has_physical: Option<bool>
}

#[get("/books")]
pub async fn get_books(state: Data<AppState>, query: web::Query<BookSearchQueryParams>) -> Result<impl Responder> {
    let include_non_physical = match query.has_physical {
        Some(true) => true,
        _ => false
    };
    match database::get_books(&state.db, 
        query.search_str.clone(), 
        query.isbn.clone(), 
        query.limit, 
        include_non_physical
        ).await {
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
    match database::get_physical_copies(&state.db, path.into_inner().0).await {
        Ok((Some(book), copies)) => Ok(web::Json(SingleBookResponse { book, copies })),
        _ => Err(actix_web::error::ErrorNotFound("Book not found")),
    }
}
