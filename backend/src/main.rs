use hll::{auth, database, routes, AppState};

use std::{env, vec};
use actix_cors::Cors;
use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::{http, HttpMessage};
use actix_web::middleware::{self, Logger};
use actix_web::{web::Data, App, HttpServer};
use actix_files;

async fn session_middleware(
    state: Data<AppState>,
    req: ServiceRequest, 
    next: middleware::Next<impl MessageBody>) 
    -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let path = req.path();
    if path == "/login_user" || path == "/register_user" {
        return next.call(req).await;
    }
    match auth::parse_auth_cookie(req.cookie(auth::AUTH_COOKIE)) {
        None => return Err(actix_web::error::ErrorUnauthorized("Could not find session token")),
        Some(token) => {
            return match auth::validate_session(&state.db, token).await {
                Ok(Some(session)) => {
                    req.extensions_mut().insert(session);
                    next.call(req).await
                },
                Ok(None) => Err(actix_web::error::ErrorUnauthorized("Session token unauthorized")),
                Err(err) => Err(actix_web::error::ErrorInternalServerError(err.to_string()))
            }
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().unwrap(); // Load .env file

    let pool = database::init_database()
        .await
        .expect("Could not initialize database");

    let frontend_url = env::var("FRONTEND_URL").unwrap();

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&frontend_url) // Frontend
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .wrap(middleware::from_fn(session_middleware))
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(routes::get_book)
            .service(routes::get_books)
            .service(routes::get_search_suggestions)
            .service(routes::register_book)
            .service(routes::add_physical_book)
            .service(routes::edit_physical_book)
            .service(routes::get_shelves)
            .service(routes::register_user)
            .service(routes::login_user)
            .service(routes::logout_user)
            .service(routes::get_user)
            .service(routes::get_user_reservations)
            .service(routes::reserve_physical_book)
            .service(routes::remove_reservation)
            .service(actix_files::Files::new("/book_cover", "./db/images/book_covers/"))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
