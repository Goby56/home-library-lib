use std::io::BufReader;
use image::{self, ImageReader};

use actix_web::{get, post, web::{self, Data}, HttpMessage, HttpRequest, Responder, Result};

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use crate::{auth::{self, Session}, database, types, AppState};

#[derive(Debug, MultipartForm)]
struct ShelveForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
    json: MpJson<types::Book>,
}

#[post("/register_book")]
pub async fn register_book(state: Data<AppState>, MultipartForm(form): MultipartForm<ShelveForm>) -> actix_web::Result<String> {
    let _book_id = match database::insert_book(&state.db, form.json.clone()).await {
        Ok(Some(book_id)) => book_id,
        Ok(None) => return Err(actix_web::error::ErrorInternalServerError("Could not create book")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };

    let reader = BufReader::new(form.file.file.reopen()?);
    let img = ImageReader::new(reader).with_guessed_format()?.decode()
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    img.save(format!("./backend/db/images/book_covers/{}.webp", form.json.isbn))
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    Ok(format!("Shelved {}. Access its cover at '/book_cover/{}.webp'", form.json.title, form.json.isbn))
}

#[derive(Deserialize)]
struct ShelfInfo {
    isbn: String,
    name: String
}

#[post("/add_physical_book")]
pub async fn add_physical_book(state: Data<AppState>, shelf_data: web::Json<ShelfInfo>) -> actix_web::Result<String> {
    let shelf = database::get_or_create_shelf(&state.db, &shelf_data.name).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    let book = database::get_books(&state.db, None, Some(&shelf_data.isbn), Some(1), true).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?.pop();
    return match (book, shelf) {
        (Some(book), Some(shelf)) => {
            database::create_physical_book(&state.db, book.id, shelf.id).await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
            Ok(format!("Added a physical copy of {} to shelf {}", book.title, shelf.name))
        },
        _ => Err(actix_web::error::ErrorNotFound("Couldn't find book or create shelf")),
    };
}


#[derive(Deserialize)]
struct EditPhysicalBookData {
    copy_id: u32,
    new_shelf_name: String
}

#[post("/edit_physical_book")] 
pub async fn edit_physical_book(state: Data<AppState>, edit_data: web::Json<EditPhysicalBookData>) -> Result<impl Responder> {
    // Can remove phyiscal book if new shelf name is left blank
    if edit_data.new_shelf_name == "" {
        match database::remove_physical_book(&state.db, edit_data.copy_id).await {
            Ok(_) => Ok(format!("Removed physical copy {}", edit_data.copy_id)),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
        }
    } else {
        match database::move_physical_book(&state.db, edit_data.copy_id, &edit_data.new_shelf_name).await {
            Ok(Some(shelf_id)) => Ok(format!("Moved physical copy {} to shelf {} ({})", edit_data.copy_id, edit_data.new_shelf_name, shelf_id)),
            Ok(None) => Err(actix_web::error::ErrorInternalServerError(format!("Could not find shelf {}", edit_data.new_shelf_name))),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
        }
    }
}

#[derive(Deserialize)]
struct PhysicalBookReservation {
    copy_id: u32,
    #[serde(with = "time::serde::iso8601")]
    start: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    end: OffsetDateTime,
}

#[post("/reserve_physical_book")] 
pub async fn reserve_physical_book(state: Data<AppState>, req: HttpRequest, reservation_data: web::Json<PhysicalBookReservation>) -> Result<impl Responder> {
    let user_id = match auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorUnauthorized("Could not find user to complete request")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };  

    match database::reserve_physical_book(&state.db, 
        user_id, reservation_data.copy_id, reservation_data.start, reservation_data.end).await {
        Ok(true) => Ok(format!("Reserved physical copy {} to user {}", reservation_data.copy_id, user_id)),
        Ok(false) => Err(actix_web::error::ErrorConflict("Reservation overlaps with another reservation")),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
}

#[post("/remove_reservation/{reservation_id}")]
pub async fn edit_reservation(state: Data<AppState>, req: HttpRequest, path: web::Path<(u32,)>) -> Result<impl Responder> {
    let user_id = match auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorUnauthorized("Could not find user to complete request")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };

    let reservation = match database::get_reservation(&state.db, path.into_inner().0).await {
        Ok(Some(reservation)) => reservation,
        Ok(None) => return Err(actix_web::error::ErrorNotFound("Could not find a reservation to edit")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };
    
    if user_id != reservation.user.id {
        return Err(actix_web::error::ErrorForbidden("User does not own reservation"))
    }

    database::remove_reservation(&state.db, reservation.id).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    Ok(format!("User {user_id} removed reservation {}", reservation.id))
}

#[derive(Serialize)]
#[serde(transparent)]
struct ReservationsResponse {
    reservations: Vec<types::Reservation>
}

#[get("/get_user_reservations")]
pub async fn get_user_reservations(state: Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let user_id = match auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorUnauthorized("Could not find user to complete request")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };
    match database::get_user_reservations(&state.db, user_id).await {
        Ok(reservations) => Ok(web::Json(ReservationsResponse { reservations })),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
}

#[derive(Serialize)]
#[serde(transparent)]
struct MultipleBooksResponse {
    books: Vec<types::Book>
}

#[derive(serde::Deserialize)]
struct BookSearchQueryParams {
    search_str: Option<String>,
    isbn: Option<String>,
    limit: Option<u32>,
    include_non_physical: Option<bool>
}

#[get("/books")]
pub async fn get_books(state: Data<AppState>, query: web::Query<BookSearchQueryParams>) -> Result<impl Responder> {
    let include_non_physical = match query.include_non_physical {
        Some(true) => true,
        _ => false
    };
    match database::get_books(&state.db, 
        query.search_str.as_deref(), 
        query.isbn.as_deref(), 
        query.limit, 
        include_non_physical
        ).await {
        Ok(books) => Ok(web::Json(MultipleBooksResponse { books })),
        _ => Ok(web::Json(MultipleBooksResponse { books: vec![] })),
    }
}

#[derive(Serialize)]
struct SingleBookResponse {
    book: types::Book,
    copies: Vec<types::PhysicalBook>
}

#[get("/book/{isbn}")]
pub async fn get_book(state: Data<AppState>, path: web::Path<(String,)>) -> Result<impl Responder> {
    match database::get_physical_copies(&state.db, &path.into_inner().0).await {
        Ok((Some(book), copies)) => Ok(web::Json(SingleBookResponse { book, copies })),
        Ok((None, _)) => Err(actix_web::error::ErrorNotFound("Book not found")),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    }
}

#[get("/get_shelves")]
pub async fn get_shelves(state: Data<AppState>) -> Result<impl Responder> {
    match database::get_shelves(&state.db).await {
        Ok(shelves) => Ok(shelves.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join(",")),
        _ => Err(actix_web::error::ErrorInternalServerError("Could not retrieve bookshelves"))
    }
}

#[derive(Deserialize)]
struct UserCredentials {
    username: String,
    password: String
}

#[derive(Serialize)]
#[serde(transparent)]
struct SessionResponse {
    token: String,
}

#[get("/get_user")]
pub async fn get_user(state: Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let extensions = req.extensions();
    let Some(session) = extensions.get::<Session>() else {
        return Err(actix_web::error::ErrorUnauthorized("Could not verify session token"));
    };
    match database::get_user(&state.db, session.user).await {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
}

#[post("/login_user")]
pub async fn login_user(state: Data<AppState>, login_data: web::Json<UserCredentials>) -> Result<impl Responder> {
    let user_id = match database::login_user(&state.db, &login_data.username, &login_data.password).await {
        Ok(Some(user_id)) => user_id,
        _ => return Err(actix_web::error::ErrorUnauthorized("User doesn't exist or incorrect password"))
    };
    match auth::create_session(&state.db, user_id).await {
        Ok(Some((_, token))) => Ok(web::Json(SessionResponse { token })),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
        _ => Err(actix_web::error::ErrorInternalServerError("Could not create session"))
    }
}

#[post("/register_user")]
pub async fn register_user(state: Data<AppState>, register_data: web::Json<UserCredentials>) -> Result<impl Responder> {
    let user_id = match database::register_user(&state.db, &register_data.username, &register_data.password).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorConflict("Username already exists")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };
    match auth::create_session(&state.db, user_id).await {
        Ok(Some((_, token))) => Ok(web::Json(SessionResponse { token })),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
        _ => Err(actix_web::error::ErrorInternalServerError("Could not create session"))
    }
}
