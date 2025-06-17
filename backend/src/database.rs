use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

use sqlx::SqlitePool;
use time::{OffsetDateTime, UtcDateTime};

use rand::{self, Rng};

use crate::types;

pub async fn get_physical_copies(pool: &SqlitePool, isbn: &str) -> Result<(Option<types::Book>, Vec<types::PhysicalBook>), sqlx::Error>  {
    let book = get_books(pool, None, Some(isbn), Some(1), true).await?.pop(); 
    // Vec should be length 0 or 1 so pop will give that element
    
    let mut physical_copies = vec![];
    if let Some(b) = &book {
        for copy_id in &b.copy_ids {
            if let Some(copy) = get_physical_book(pool, *copy_id).await? {
                physical_copies.push(copy); 
            }
        }
    }
    Ok((book, physical_copies))
}

async fn get_physical_book(pool: &SqlitePool, id: u32) -> Result<Option<types::PhysicalBook>, sqlx::Error> {
    let physical_copy: (u32, Option<String>) = sqlx::query_as("
        SELECT 
            PhysicalBook.shelf,
            GROUP_CONCAT(DISTINCT Reservation.id) AS reservations
        FROM PhysicalBook
        LEFT JOIN BookReservationMatch ON PhysicalBook.id = BookReservationMatch.physical_book
        LEFT JOIN Reservation ON BookReservationMatch.reservation = Reservation.id
        WHERE PhysicalBook.id = ?
        GROUP BY PhysicalBook.shelf").bind(id).fetch_one(pool).await?;

    let Some(shelf) = get_shelf(pool, Some(physical_copy.0), None).await? else {
        return Ok(None);
    };

    let mut reservations = vec![];
        
    if let Some(reservations_str) = physical_copy.1 {
        for reservation_id in reservations_str.split(",").filter_map(|s| s.trim().parse::<u32>().ok()) {
            if let Some(reservation) = get_reservation(pool, reservation_id).await? {
                reservations.push(reservation);
            }
        }
    }

    Ok(Some(types::PhysicalBook {
        id, shelf, reservations
    }))
}

pub async fn move_physical_book(pool: &SqlitePool, id: u32, new_shelf: &str) -> Result<Option<u32>, sqlx::Error> {
    let shelf: Option<types::Shelf> = get_or_create_shelf(pool, new_shelf).await?;
    if let Some(shelf) = shelf {
        sqlx::query("
            UPDATE PhysicalBook
            SET shelf = ?
            WHERE id = ?").bind(shelf.id).bind(id).execute(pool).await?;
        return Ok(Some(shelf.id))
    }
    Ok(None)
}

pub async fn remove_physical_book(pool: &SqlitePool, id: u32) -> Result<(), sqlx::Error> {
    sqlx::query("
        DELETE FROM PhysicalBook
        WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn reserve_physical_book(pool: &SqlitePool, user_id: u32, copy_id: u32, start_date: OffsetDateTime, end_date: OffsetDateTime) -> Result<bool, sqlx::Error> {
    let Some(physical_copy) = get_physical_book(pool, copy_id).await? else {
        return Ok(false);
    };
    
    for reservation in physical_copy.reservations {
        if reservation.intersects(start_date, end_date) {
            return Ok(false);
        }
    }
    let now = UtcDateTime::now();
    let reservation_id: u32 = sqlx::query_scalar("
        INSERT INTO Reservation (user, created_at, start_date, end_date)
        VALUES (?, ?, ?, ?)
        RETURNING id").bind(user_id).bind(now.unix_timestamp()).bind(start_date).bind(end_date)
        .fetch_one(pool).await?;

    sqlx::query("
        INSERT INTO BookReservationMatch (physical_book, reservation)
        VALUES (?, ?)").bind(copy_id).bind(reservation_id).execute(pool).await?;

    Ok(true)
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct ReservationIntermediate {
    pub id: u32,
    pub user: u32,
    pub created_at: i64,
    #[serde(with = "time::serde::iso8601")]
    pub start_date:  OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
}

pub async fn get_reservation(pool: &SqlitePool, id: u32) -> Result<Option<types::Reservation>, sqlx::Error> {
    let reservation: Option<ReservationIntermediate> = sqlx::query_as("
        SELECT id, user, created_at, start_date, end_date
        FROM Reservation
        WHERE id = ?").bind(id).fetch_optional(pool).await?;
    let Some(reservation) = reservation else {
        return Ok(None);
    };
    let user = get_user(pool, reservation.user).await?;

    Ok(Some(types::Reservation {
        id: reservation.id, 
        user, 
        created_at: reservation.created_at, 
        start_date: reservation.start_date,
        end_date: reservation.end_date
    }))
}

pub async fn remove_reservation(pool: &SqlitePool, id: u32) -> Result<(), sqlx::Error> {
    sqlx::query("
        DELETE FROM Reservation
        WHERE id = ?").bind(id).execute(pool).await?;
    Ok(())
}

pub async fn get_shelves(pool: &SqlitePool) -> Result<Vec<types::Shelf>, sqlx::Error> {
    let shelves: Vec<types::Shelf> = sqlx::query_as("
        SELECT id, name FROM Shelf
        ").fetch_all(pool).await?;
    return Ok(shelves);
}

pub async fn get_shelf(pool: &SqlitePool, id: Option<u32>, name: Option<&str>) -> Result<Option<types::Shelf>, sqlx::Error> {
    if let Some(id) = id {
        let shelf: Option<types::Shelf> = sqlx::query_as("
            SELECT id, name 
            FROM Shelf 
            WHERE id = ?").bind(id).fetch_optional(pool).await?;
        return Ok(shelf);
    }
    if let Some(name) = name {
        let shelf: Option<types::Shelf> = sqlx::query_as("
            SELECT id, name 
            FROM Shelf 
            WHERE name = ?").bind(name).fetch_optional(pool).await?;
        return Ok(shelf);
    }
    return Ok(None)
}


pub async fn get_or_create_shelf(pool: &SqlitePool, name: &str) -> Result<Option<types::Shelf>, sqlx::Error> {
    let shelf_id: Option<u32> = sqlx::query_scalar("
        INSERT INTO Shelf (name) VALUES (?)
        ON CONFLICT(name) DO UPDATE SET name=name
        RETURNING id").bind(name).fetch_optional(pool).await?;
    if let Some(id) = shelf_id {
        return Ok(Some(types::Shelf {
            id, name: name.to_string()
        }));
    }
    Ok(None)
}

pub async fn create_physical_book(pool: &SqlitePool, book: u32, shelf: u32) -> Result<(), sqlx::Error> {
    sqlx::query("
        INSERT INTO PhysicalBook (book, shelf)
        VALUES (?, ?)")
        .bind(book).bind(shelf).execute(pool).await?;
    Ok(())
}

pub async fn insert_book(pool: &SqlitePool, book: types::Book) -> Result<Option<u32>, sqlx::Error> {
    let book_id: Option<u32> = sqlx::query_scalar("
        INSERT INTO Book (isbn, title, publication_year, page_count, language)
        VALUES (?, ?, ?, ?, ?) RETURNING id")
        .bind(book.isbn)
        .bind(book.title)
        .bind(book.publication_year)
        .bind(book.page_count)
        .bind(book.language)
        .fetch_optional(pool).await?;

    let Some(book_id) = book_id else {
        return Ok(None);
    };
    
    for author in book.authors { // Insert new author and connect to book
        let author_id: Option<u32> = sqlx::query_scalar("
            INSERT INTO Author (name) VALUES (?)
            ON CONFLICT(name) DO UPDATE SET name=excluded.name
            RETURNING id")
            .bind(author)
            .fetch_optional(pool).await?;
        if let Some(id) = author_id {
            sqlx::query("
                INSERT INTO BookContribution (book, author) VALUES (?, ?)")
                .bind(book_id)
                .bind(id)
                .execute(pool).await?;
        }
    }

    for genre in book.genres { // Insert genre and genre connection to book
        let genre_id: Option<u32> = sqlx::query_scalar("
            INSERT INTO Genre (name) VALUES (?)
            ON CONFLICT(name) DO UPDATE SET name=excluded.name
            RETURNING id")
            .bind(genre)
            .fetch_optional(pool).await?;
        if let Some(id) = genre_id {
            sqlx::query("
                INSERT INTO GenreMatch (book, genre) VALUES (?, ?)")
                .bind(book_id)
                .bind(id)
                .execute(pool).await?;
        }
    }
    Ok(Some(book_id))
}

pub async fn get_books(pool: &SqlitePool, _search_str: Option<&str>, 
    isbn: Option<&str>, limit: Option<u32>,
    include_non_physical: bool) 
    -> Result<Vec<types::Book>, sqlx::Error>{
    let mut sq = format!(r#"
        SELECT 
            Book.id,
            Book.title,
            Book.isbn,
            GROUP_CONCAT(DISTINCT Author.name) AS authors,
            Book.publication_year,
            GROUP_CONCAT(DISTINCT Genre.name) AS genres,
            Book.page_count,
            Book.language,
            GROUP_CONCAT(DISTINCT PhysicalBook.id) as copies
        FROM Book
        {}JOIN PhysicalBook ON Book.id = PhysicalBook.book
        JOIN BookContribution ON Book.id = BookContribution.book
        JOIN Author ON BookContribution.author = Author.id
        JOIN GenreMatch ON Book.id = GenreMatch.book
        JOIN Genre ON GenreMatch.genre = Genre.id
        "#, match include_non_physical {
                true => "LEFT ",
                false => ""
            });

    if isbn.is_some() {
        sq.push_str(" WHERE Book.isbn = ?");
    }

    sq.push_str(" GROUP BY Book.id");

    if limit.is_some() {
        sq.push_str(" LIMIT ?");
    }

    sq.push(';');

    let mut query = sqlx::query_as(&sq);
    if let Some(isbn) = isbn {
        query = query.bind(isbn);
    }
    if let Some(limit) = limit {
        query = query.bind(limit);
    }
    let books: Vec<(u32, String, String, String, i16, String, u16, String, Option<String>)> = query.fetch_all(pool).await?;
    
    return Ok(books.into_iter().map(|b| {
        types::Book {
            id: b.0,
            title: b.1,
            isbn: b.2,
            authors: b.3.split(",").map(|s| s.to_string()).collect(),
            publication_year: b.4,
            genres: b.5.split(",").map(|s| s.to_string()).collect(),
            page_count: b.6,
            language: b.7,
            copy_ids: match b.8 {
                Some(s) => s.split(",").filter_map(|s| s.trim().parse::<u32>().ok()).collect(),
                None => vec![]
            }
    }

    }).collect())
}

pub async fn get_user(pool: &SqlitePool, id: u32) -> Result<types::User, sqlx::Error> {
    let user: types::User = sqlx::query_as("
        SELECT id, username, personal_color
        FROM User
        WHERE id = ?").bind(id).fetch_one(pool).await?;
    Ok(user)
}

pub async fn login_user(pool: &SqlitePool, username: &str, password: &str) -> Result<Option<u32>, sqlx::Error> {
    let (id, password_hash): (u32, String) = sqlx::query_as("
        SELECT id, password_hash
        FROM User
        WHERE username = ?").bind(username).fetch_one(pool).await?;
    if let Ok(parsed_hash) = PasswordHash::new(&password_hash) {
        if Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok() {
            return Ok(Some(id))
        }
    }
    Ok(None)
}

pub async fn register_user(pool: &SqlitePool, username: &str, password: &str) -> Result<Option<u32>, sqlx::Error> {
    let salt = SaltString::generate(&mut OsRng);
    
    let rgb: [u8;3] = rand::rng().random();
    let personal_color = hex::encode(rgb);

    if let Ok(password_hash) = Argon2::default().hash_password(password.as_bytes(), &salt) {
        let user_id: Option<u32> = sqlx::query_scalar("
            INSERT INTO User (username, password_hash, personal_color)
            VALUES (?, ?, ?)
            ON CONFLICT (username) DO NOTHING
            RETURNING id").bind(username).bind(password_hash.to_string()).bind(personal_color).fetch_optional(pool).await?;
        return Ok(user_id);
    }
    Ok(None)
}
