use std::{fs, io::BufReader, path::PathBuf};
use image::{self, ImageReader};

use actix_web::{get, post, web::{self, Data}, HttpMessage, HttpRequest, Responder, Result};

use actix_multipart::form::{json::Json as MpJson, tempfile::TempFile, MultipartForm};
use serde::{Serialize, Deserialize};
use serde_with::rust::double_option;
use time::OffsetDateTime;
use uuid::Uuid;
use crate::{auth::{self, Session}, database::{crud, search}, types, AppState};

#[derive(Debug, MultipartForm)]
struct BookAndCoverForm {
    #[multipart(limit = "100MB")]
    cover: Option<TempFile>,
    book: MpJson<BookForm>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BookForm {
    #[serde(default, with = "double_option")]
    pub isbn: Option<Option<String>>,
    pub title: Option<String>,
    pub authors: Option<String>,
    #[serde(default, with = "double_option")]
    pub genres: Option<Option<String>>,
    #[serde(default, with = "double_option")]
    pub publication_year: Option<Option<i16>>,
    #[serde(default, with = "double_option")]
    pub page_count: Option<Option<u16>>,
    #[serde(default, with = "double_option")]
    pub language: Option<Option<String>>,
}

#[post("/register_book")]
pub async fn register_book(state: Data<AppState>, MultipartForm(form): MultipartForm<BookAndCoverForm>) -> actix_web::Result<String> {
    let uuid = match crud::insert_book(&state.db, form.book.clone()).await {
        Ok(Some(uuid)) => uuid.to_string(),
        Ok(None) => return Err(actix_web::error::ErrorInternalServerError("Title and authors has to be provided")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };

    if let Some(file) = form.cover {
        let reader = BufReader::new(file.file.reopen()?);
        let img = ImageReader::new(reader).with_guessed_format()?.decode()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
        img.save(format!("./db/images/book_covers/{}.webp", uuid.clone()))
            .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    }
    
    // Return the preferred identifier of the book
    Ok(form.book.isbn.clone().flatten().unwrap_or_else(|| uuid.to_string()))
}

#[post("/edit_book/{book_uuid}")]
pub async fn edit_book(state: Data<AppState>, MultipartForm(form): MultipartForm<BookAndCoverForm>, path: web::Path<(Uuid,)>) -> actix_web::Result<String> {
    let uuid = path.into_inner().0;
    let _ = crud::edit_book(&state.db, uuid, form.book.clone()).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()));

    if let Some(file) = form.cover {
        let reader = BufReader::new(file.file.reopen()?);
        let img = ImageReader::new(reader).with_guessed_format()?.decode()
            .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
        img.save(format!("./db/images/book_covers/{}.webp", uuid.to_string()))
            .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    }
    
    // Return the preferred identifier of the book
    Ok(form.book.isbn.clone().flatten().unwrap_or_else(|| uuid.to_string()))
}

#[post("/delete_book/{book_uuid}")]
pub async fn delete_book(state: Data<AppState>, path: web::Path<(Uuid,)>) -> Result<impl Responder> {
    let uuid = path.into_inner().0;
    match crud::delete_book(&state.db, uuid).await {
        Ok(()) => {},
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
    let file_path: PathBuf = format!("./db/images/book_covers/{}.webp", uuid.to_string()).into();
    if file_path.exists() {
        let _ = fs::remove_file(file_path);
    }
    Ok(format!("Deleted book with UUID {}", uuid.to_string()))
}

#[derive(Deserialize)]
struct ShelfInfo {
    uuid: Uuid,
    name: String
}

#[post("/add_physical_book")]
pub async fn add_physical_book(state: Data<AppState>, shelf_data: web::Json<ShelfInfo>) -> actix_web::Result<String> {
    let shelf = crud::get_shelf(&state.db, None, Some(&shelf_data.name)).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    let book = crud::get_book(&state.db, None, Some(shelf_data.uuid)).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    return match (book, shelf) {
        (book, Some(shelf)) => {
            crud::create_physical_book(&state.db, book.id, shelf.id).await
                .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
            Ok(format!("Added a physical copy of {} to shelf {}", book.title, shelf.name))
        },
        _ => Err(actix_web::error::ErrorNotFound("Couldn't find book or shelf")),
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
        match crud::remove_physical_book(&state.db, edit_data.copy_id).await {
            Ok(_) => Ok(format!("Removed physical copy {}", edit_data.copy_id)),
            Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
        }
    } else {
        match crud::move_physical_book(&state.db, edit_data.copy_id, &edit_data.new_shelf_name).await {
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

    match crud::reserve_physical_book(&state.db, 
        user_id, reservation_data.copy_id, reservation_data.start, reservation_data.end).await {
        Ok(true) => Ok(format!("Reserved physical copy {} to user {}", reservation_data.copy_id, user_id)),
        Ok(false) => Err(actix_web::error::ErrorConflict("Reservation overlaps with another reservation")),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
}

#[post("/remove_reservation/{reservation_id}")]
pub async fn remove_reservation(state: Data<AppState>, req: HttpRequest, path: web::Path<(u32,)>) -> Result<impl Responder> {
    let user_id = match auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorUnauthorized("Could not find user to complete request")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };

    let reservation = match crud::get_reservation(&state.db, path.into_inner().0).await {
        Ok(Some(reservation)) => reservation,
        Ok(None) => return Err(actix_web::error::ErrorNotFound("Could not find the reservation to remove")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };
    
    if user_id != reservation.user.id {
        return Err(actix_web::error::ErrorForbidden("User does not own reservation"))
    }

    crud::remove_reservation(&state.db, reservation.id).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
    Ok(format!("User {user_id} removed reservation {}", reservation.id))
}

#[derive(Serialize)]
#[serde(transparent)]
struct BookReservationsResponse {
    reservations: Vec<crud::BookReservation>
}

#[get("/get_user_reservations")]
pub async fn get_user_reservations(state: Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let user_id = match auth::get_user_from_cookie(&state.db, req.cookie(auth::AUTH_COOKIE)).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorUnauthorized("Could not find user to complete request")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };

    let reservations = match crud::get_user_reservations(&state.db, user_id).await {
        Ok(reservations) => reservations,
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    };
    
    let mut book_reservations = vec![];
    
    for rsv in reservations {
        match crud::get_book_reservation(&state.db, rsv).await {
            Ok(Some(reservation)) => book_reservations.push(reservation),
            Ok(None) => {},
            Err(err) => return Err(actix_web::error::ErrorInternalServerError(err.to_string()))
        };
    }

    Ok(web::Json(BookReservationsResponse { reservations: book_reservations }))
}

#[derive(Serialize)]
#[serde(transparent)]
struct MultipleBooksResponse {
    books: Vec<types::Book>
}

#[derive(serde::Deserialize)]
struct BookSearchQueryParams {
    search_str: Option<String>,
    limit: Option<u32>,
    only_physical: Option<bool>
}

#[get("/books")]
pub async fn get_books(state: Data<AppState>, query: web::Query<BookSearchQueryParams>) -> Result<impl Responder> {
    let only_physical = match query.only_physical {
        Some(false) => false,
        _ => true
    };
    
    let mut search_str = None;

    if let Some(search_param) = query.search_str.clone() {
        let spellfix_candidates = search::get_spelling_candidates(&state.db, &search_param, 1).await
            .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;
        
        if let Some(c) = spellfix_candidates {
            search_str = Some(c.get_top_candidate());
        };
    };

    match crud::query_books(&state.db, 
        search_str.as_deref(), 
        query.limit, 
        only_physical
        ).await {
        Ok(books) => Ok(web::Json(MultipleBooksResponse { books })),
        _ => Ok(web::Json(MultipleBooksResponse { books: vec![] })),
    }
}

#[derive(Serialize)]
#[serde(transparent)]
struct SearchSuggestions {
    suggestions: Vec<types::BookSearchSuggestion>
}

#[get("/get_search_suggestions")]
pub async fn get_search_suggestions(state: Data<AppState>, query: web::Query<BookSearchQueryParams>) -> Result<impl Responder> {
    let Some(search_param) = query.search_str.as_deref() else {
        return Err(actix_web::error::ErrorBadRequest("Query parameter 'search_str' is required"));
    };

    let spellfix_candidates = search::get_spelling_candidates(&state.db, search_param, 1).await
        .map_err(|err| actix_web::error::ErrorInternalServerError(err.to_string()))?;

    let search_str = match spellfix_candidates {
        Some(c) => c.get_top_candidate(),
        None => search_param.to_string()
    };

    match crud::search_suggestions(&state.db, &search_str).await {
        Ok(suggestions) => Ok(web::Json(SearchSuggestions { suggestions })),
        _ => Ok(web::Json(SearchSuggestions { suggestions: vec![] })),
    }

}

#[derive(Serialize)]
struct SingleBookResponse {
    book: types::Book,
    copies: Vec<types::PhysicalBook>
}

#[get("/book/{identifier}")]
pub async fn get_book(state: Data<AppState>, path: web::Path<(String,)>) -> Result<impl Responder> {
    match crud::get_physical_copies(&state.db, path.into_inner().0).await {
        Ok((book, copies)) => Ok(web::Json(SingleBookResponse { book, copies })),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
    }
}

#[get("/get_shelves")]
pub async fn get_shelves(state: Data<AppState>) -> Result<impl Responder> {
    match crud::get_shelves(&state.db).await {
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
    match crud::get_user(&state.db, session.user).await {
        Ok(user) => Ok(web::Json(user)),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
}

#[post("/logout_user")]
pub async fn logout_user(state: Data<AppState>, req: HttpRequest) -> Result<impl Responder> {
    let extensions = req.extensions();
    let Some(session) = extensions.get::<Session>() else {
        return Err(actix_web::error::ErrorUnauthorized("Could not verify session token"));
    };
    match auth::invalidate_session(&state.db, session).await {
        Ok(()) => Ok("Logged user out"),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
    }
}


#[post("/login_user")]
pub async fn login_user(state: Data<AppState>, login_data: web::Json<UserCredentials>) -> Result<impl Responder> {
    let user_id = match crud::login_user(&state.db, &login_data.username, &login_data.password).await {
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
    let user_id = match crud::register_user(&state.db, &register_data.username, &register_data.password).await {
        Ok(Some(user_id)) => user_id,
        Ok(None) => return Err(actix_web::error::ErrorConflict("Username already exists")),
        Err(err) => return Err(actix_web::error::ErrorInternalServerError(format!("Could not register user: {err}")))
    };
    match auth::create_session(&state.db, user_id).await {
        Ok(Some((_, token))) => Ok(web::Json(SessionResponse { token })),
        Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string())),
        _ => Err(actix_web::error::ErrorInternalServerError("Could not create session"))
    }
}

#[derive(Deserialize)]
struct NewStringQueryParam {
    new: String
}

#[post("/change_username")]
pub async fn change_username(state: Data<AppState>, req: HttpRequest, query: web::Query<NewStringQueryParam>) -> Result<impl Responder> {
    let extensions = req.extensions();
    let Some(session) = extensions.get::<Session>() else {
        return Err(actix_web::error::ErrorUnauthorized("Could not verify session token"));
    };
    match crud::change_username(&state.db, session.user, &query.new).await {
        Ok(old) => Ok(format!("Changed username from {old} to {}", query.new)),
        _ => Err(actix_web::error::ErrorConflict("Username may already exist"))
    }
}

#[post("/change_personal_color")]
pub async fn change_personal_color(state: Data<AppState>, req: HttpRequest, query: web::Query<NewStringQueryParam>) -> Result<impl Responder> {
    let extensions = req.extensions();
    let Some(session) = extensions.get::<Session>() else {
        return Err(actix_web::error::ErrorUnauthorized("Could not verify session token"));
    };
    match crud::change_personal_color(&state.db, session.user, &query.new).await {
        Ok(old) => Ok(format!("Changed personal color from {old} to {}", query.new)),
        _ => Err(actix_web::error::ErrorConflict("Color already taken"))
    }
}
