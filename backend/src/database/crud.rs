use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use serde::Serialize;
use sqlx::SqlitePool;
use time::{OffsetDateTime, UtcDateTime};

use rand::{self, Rng};
use uuid::Uuid;

use crate::{routes, types};

pub async fn get_physical_copies(
    pool: &SqlitePool,
    identifier: String,
) -> Result<(types::Book, Vec<types::PhysicalBook>), sqlx::Error> {
    let book = match Uuid::parse_str(&identifier) {
        Ok(uuid) => get_book(pool, None, Some(uuid)).await?,
        Err(_) => get_book(pool, Some(&identifier), None).await?
    };

    // Vec should be length 0 or 1 so pop will give that element
    let mut physical_copies = vec![];
    for copy_id in &book.copy_ids {
        if let Some(copy) = get_physical_book(pool, *copy_id).await? {
            physical_copies.push(copy);
        }
    }
    Ok((book, physical_copies))
}

async fn get_physical_book(
    pool: &SqlitePool,
    id: u32,
) -> Result<Option<types::PhysicalBook>, sqlx::Error> {
    let physical_copy: (u32, Option<String>) = sqlx::query_as(
        "
        SELECT 
            PhysicalBook.shelf,
            GROUP_CONCAT(DISTINCT Reservation.id) AS reservations
        FROM PhysicalBook
        LEFT JOIN BookReservationMatch ON PhysicalBook.id = BookReservationMatch.physical_book
        LEFT JOIN Reservation ON BookReservationMatch.reservation = Reservation.id
        WHERE PhysicalBook.id = ?
        GROUP BY PhysicalBook.shelf",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    let Some(shelf) = get_shelf(pool, Some(physical_copy.0), None).await? else {
        return Ok(None);
    };

    let mut reservations = vec![];

    if let Some(reservations_str) = physical_copy.1 {
        for reservation_id in reservations_str
            .split(",")
            .filter_map(|s| s.trim().parse::<u32>().ok())
        {
            if let Some(reservation) = get_reservation(pool, reservation_id).await? {
                reservations.push(reservation);
            }
        }
    }

    Ok(Some(types::PhysicalBook {
        id,
        shelf,
        reservations,
    }))
}

#[derive(Serialize)]
pub struct BookReservation {
    pub isbn: String,
    pub title: String,
    pub copy_id: u32,
    pub shelf: types::Shelf,
    pub reservation: types::Reservation,
}

pub async fn get_book_reservation(
    pool: &SqlitePool,
    reservation: types::Reservation,
) -> Result<Option<BookReservation>, sqlx::Error> {
    let book_info: Option<(String, String, u32, u32)> = sqlx::query_as(
        "
        SELECT 
            Book.isbn,
            Book.title,
            PhysicalBook.id,
            PhysicalBook.shelf
        FROM PhysicalBook
        LEFT JOIN BookReservationMatch ON PhysicalBook.id = BookReservationMatch.physical_book
        LEFT JOIN Reservation ON BookReservationMatch.reservation = Reservation.id
        LEFT JOIN Book ON PhysicalBook.book = Book.id
        WHERE Reservation.id = ?
        GROUP BY PhysicalBook.shelf",
    )
    .bind(reservation.id)
    .fetch_optional(pool)
    .await?;

    let Some(book_info) = book_info else {
        return Ok(None);
    };

    let Some(shelf) = get_shelf(pool, Some(book_info.3), None).await? else {
        return Ok(None);
    };

    Ok(Some(BookReservation {
        isbn: book_info.0,
        title: book_info.1,
        copy_id: book_info.2,
        shelf,
        reservation,
    }))
}

pub async fn move_physical_book(
    pool: &SqlitePool,
    id: u32,
    new_shelf: &str,
) -> Result<Option<u32>, sqlx::Error> {
    let shelf: Option<types::Shelf> = get_or_create_shelf(pool, new_shelf).await?;
    if let Some(shelf) = shelf {
        sqlx::query(
            "
            UPDATE PhysicalBook
            SET shelf = ?
            WHERE id = ?",
        )
        .bind(shelf.id)
        .bind(id)
        .execute(pool)
        .await?;
        return Ok(Some(shelf.id));
    }
    Ok(None)
}

pub async fn remove_physical_book(pool: &SqlitePool, id: u32) -> Result<(), sqlx::Error> {
    let physical_book = get_physical_book(pool, id).await?;
    if let Some(physical_book) = physical_book {
        sqlx::query(
            "
            DELETE FROM PhysicalBook
            WHERE id = ?",
        )
        .bind(id)
        .execute(pool)
        .await?;
        for reservation in physical_book.reservations {
            // Separate deletions, may be slow, but this is on a small scale
            remove_reservation(pool, reservation.id).await?;
        }
    }
    Ok(())
}

pub async fn reserve_physical_book(
    pool: &SqlitePool,
    user_id: u32,
    copy_id: u32,
    start_date: OffsetDateTime,
    end_date: OffsetDateTime,
) -> Result<bool, sqlx::Error> {
    let Some(physical_copy) = get_physical_book(pool, copy_id).await? else {
        return Ok(false);
    };
    
    if start_date < OffsetDateTime::now_utc() {
        return Ok(false);
    }

    for reservation in physical_copy.reservations {
        if reservation.intersects(start_date, end_date) {
            return Ok(false);
        }
    }
    let now = UtcDateTime::now();
    let reservation_id: u32 = sqlx::query_scalar(
        "
        INSERT INTO Reservation (user, created_at, start_date, end_date)
        VALUES (?, ?, ?, ?)
        RETURNING id",
    )
    .bind(user_id)
    .bind(now.unix_timestamp())
    .bind(start_date)
    .bind(end_date)
    .fetch_one(pool)
    .await?;

    sqlx::query(
        "
        INSERT INTO BookReservationMatch (physical_book, reservation)
        VALUES (?, ?)",
    )
    .bind(copy_id)
    .bind(reservation_id)
    .execute(pool)
    .await?;

    Ok(true)
}

#[derive(sqlx::FromRow, serde::Serialize)]
struct ReservationIntermediate {
    pub id: u32,
    pub user: u32,
    pub created_at: i64,
    #[serde(with = "time::serde::iso8601")]
    pub start_date: OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
}

pub async fn get_reservation(
    pool: &SqlitePool,
    id: u32,
) -> Result<Option<types::Reservation>, sqlx::Error> {
    let reservation: Option<ReservationIntermediate> = sqlx::query_as(
        "
        SELECT id, user, created_at, start_date, end_date
        FROM Reservation
        WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;
    let Some(reservation) = reservation else {
        return Ok(None);
    };
    let user = get_user(pool, reservation.user).await?;

    Ok(Some(types::Reservation {
        id: reservation.id,
        user,
        created_at: reservation.created_at,
        start_date: reservation.start_date,
        end_date: reservation.end_date,
    }))
}

pub async fn remove_reservation(pool: &SqlitePool, id: u32) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        DELETE FROM Reservation
        WHERE id = ?",
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_shelves(pool: &SqlitePool) -> Result<Vec<types::Shelf>, sqlx::Error> {
    let shelves: Vec<types::Shelf> = sqlx::query_as(
        "
        SELECT id, name FROM Shelf
        ",
    )
    .fetch_all(pool)
    .await?;
    return Ok(shelves);
}

pub async fn get_shelf(
    pool: &SqlitePool,
    id: Option<u32>,
    name: Option<&str>,
) -> Result<Option<types::Shelf>, sqlx::Error> {
    if let Some(id) = id {
        let shelf: Option<types::Shelf> = sqlx::query_as(
            "
            SELECT id, name 
            FROM Shelf 
            WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        return Ok(shelf);
    }
    if let Some(name) = name {
        let shelf: Option<types::Shelf> = sqlx::query_as(
            "
            SELECT id, name 
            FROM Shelf 
            WHERE name = ?",
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        return Ok(shelf);
    }
    return Ok(None);
}

pub async fn get_or_create_shelf(
    pool: &SqlitePool,
    name: &str,
) -> Result<Option<types::Shelf>, sqlx::Error> {
    let shelf_id: Option<u32> = sqlx::query_scalar(
        "
        INSERT INTO Shelf (name) VALUES (?)
        ON CONFLICT(name) DO UPDATE SET name=name
        RETURNING id",
    )
    .bind(name)
    .fetch_optional(pool)
    .await?;
    if let Some(id) = shelf_id {
        return Ok(Some(types::Shelf {
            id,
            name: name.to_string(),
        }));
    }
    Ok(None)
}

pub async fn create_physical_book(
    pool: &SqlitePool,
    book: u32,
    shelf: u32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "
        INSERT INTO PhysicalBook (book, shelf)
        VALUES (?, ?)",
    )
    .bind(book)
    .bind(shelf)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn insert_book(pool: &SqlitePool, book: routes::BookForm) -> Result<Uuid, sqlx::Error> {
    let uuid = Uuid::new_v4();
    sqlx::query("
        INSERT INTO Book (uuid, isbn, title, authors, genres, publication_year, page_count, language)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?)",)
    .bind(uuid)
    .bind(book.isbn)
    .bind(book.title)
    .bind(book.authors)
    .bind(book.genres)
    .bind(book.publication_year)
    .bind(book.page_count)
    .bind(book.language).execute(pool).await?;

    Ok(uuid)
}

#[derive(sqlx::FromRow)]
struct BookIntermediate {
    id: u32,
    uuid: Uuid,
    isbn: Option<String>,
    title: String,
    authors: String,
    genres: Option<String>,
    publication_year: Option<i16>,
    page_count: Option<u16>,
    language: Option<String>,
    copies: Option<String>
}

impl BookIntermediate {
    fn to_book(&self) -> types::Book {
        types::Book {
            uuid: self.uuid,
            id: self.id,
            title: self.title.clone(),
            isbn: self.isbn.clone(),
            authors: self.authors.lines().map(|s| s.to_string()).collect(),
            genres: match &self.genres {
                Some(s) => s.lines().map(|s| s.to_string()).collect(),
                None => vec![],
            },
            publication_year: self.publication_year,
            page_count: self.page_count,
            language: self.language.clone(),
            copy_ids: match &self.copies {
                Some(s) => s
                    .split(",")
                    .filter_map(|s| s.trim().parse::<u32>().ok())
                    .collect(),
                None => vec![],
            }
        }
    }
}

pub async fn get_book(pool: &SqlitePool, isbn: Option<&str>, uuid: Option<Uuid>) -> Result<types::Book, sqlx::Error> {
    let book: BookIntermediate = sqlx::query_as("
        SELECT 
            Book.id as id,
            Book.uuid,
            Book.isbn,
            Book.title,
            Book.authors,
            Book.genres,
            Book.publication_year,
            Book.page_count,
            Book.language,
            GROUP_CONCAT(DISTINCT PhysicalBook.id) as copies
        FROM Book
        LEFT JOIN PhysicalBook ON Book.id = PhysicalBook.book
        WHERE Book.isbn = ? OR Book.uuid = ?
        GROUP BY Book.id",
    ).bind(isbn).bind(uuid).fetch_one(pool).await?;

    Ok(book.to_book())
}

pub async fn query_books(
    pool: &SqlitePool,
    search_str: Option<&str>,
    limit: Option<u32>,
    only_physical: bool,
) -> Result<Vec<types::Book>, sqlx::Error> {
    let sq = format!("
        WITH RankedBooks AS (
            SELECT 
                *,
                bm25(BookFts, 0, 8, 4, 2) AS rank
            FROM Book
            INNER JOIN BookFts ON BookFts.book_id = Book.id
            {}
            ORDER BY rank
            LIMIT ?
        )
        SELECT 
            RankedBooks.id,
            RankedBooks.uuid,
            RankedBooks.isbn,
            RankedBooks.title,
            RankedBooks.authors,
            RankedBooks.genres,
            RankedBooks.publication_year,
            RankedBooks.page_count,
            RankedBooks.language,
            GROUP_CONCAT(DISTINCT PhysicalBook.id) AS copies
        FROM RankedBooks
        {}JOIN PhysicalBook ON PhysicalBook.book = RankedBooks.id
        GROUP BY RankedBooks.id
        ORDER BY MIN(RankedBooks.rank);
        ",
        match search_str {
           Some(_) => "WHERE BookFts MATCH ?",
           None => ""
        },
        match only_physical {
            true => "",
            false => "LEFT ",
        }
    );
    let mut query = sqlx::query_as(&sq);
    if let Some(search_str) = search_str {
        query = query.bind(search_str);
    }
    let books: Vec<BookIntermediate> = query.bind(limit.unwrap_or(20)).fetch_all(pool).await?;

    return Ok(books
        .into_iter()
        .map(|b| b.to_book())
        .collect());
}

pub async fn search_suggestions(
    pool: &SqlitePool,
    search_str: &str,
) -> Result<Vec<types::BookSearchSuggestion>, sqlx::Error> {
    let suggestions: Vec<types::BookSearchSuggestion> = sqlx::query_as("
        SELECT 
            Book.uuid,
            Book.isbn,
            Book.title,
            Book.authors,
            Book.genres
        FROM Book
        INNER JOIN BookFts ON BookFts.book_id = Book.id
        WHERE BookFts MATCH ?
        ORDER BY bm25(BookFts, 0, 8, 4, 2)
        LIMIT 15").bind(search_str).fetch_all(pool).await?;

    Ok(suggestions)
}

pub async fn get_user_reservations(
    pool: &SqlitePool,
    user_id: u32,
) -> Result<Vec<types::Reservation>, sqlx::Error> {
    let user = get_user(pool, user_id).await?;
    let reservations: Vec<ReservationIntermediate> = sqlx::query_as(
        "
        SELECT id, user, created_at, start_date, end_date
        FROM Reservation
        WHERE user = ?",
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;
    Ok(reservations
        .iter()
        .map(|rsv| types::Reservation {
            id: rsv.id,
            user: user.clone(),
            created_at: rsv.created_at,
            start_date: rsv.start_date,
            end_date: rsv.end_date,
        })
        .collect())
}

pub async fn get_user(pool: &SqlitePool, id: u32) -> Result<types::User, sqlx::Error> {
    let user: types::User = sqlx::query_as(
        "
        SELECT id, username, personal_color
        FROM User
        WHERE id = ?",
    )
    .bind(id)
    .fetch_one(pool)
    .await?;
    Ok(user)
}

pub async fn login_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<Option<u32>, sqlx::Error> {
    let (id, password_hash): (u32, String) = sqlx::query_as(
        "
        SELECT id, password_hash
        FROM User
        WHERE username = ?",
    )
    .bind(username)
    .fetch_one(pool)
    .await?;
    if let Ok(parsed_hash) = PasswordHash::new(&password_hash) {
        if Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
        {
            return Ok(Some(id));
        }
    }
    Ok(None)
}

pub async fn register_user(
    pool: &SqlitePool,
    username: &str,
    password: &str,
) -> Result<Option<u32>, sqlx::Error> {
    let salt = SaltString::generate(&mut OsRng);

    let rgb: [u8; 3] = rand::rng().random();
    let personal_color = hex::encode(rgb);

    if let Ok(password_hash) = Argon2::default().hash_password(password.as_bytes(), &salt) {
        let user_id: Option<u32> = sqlx::query_scalar(
            "
            INSERT INTO User (username, password_hash, personal_color)
            VALUES (?, ?, ?)
            ON CONFLICT (username) DO NOTHING
            RETURNING id",
        )
        .bind(username)
        .bind(password_hash.to_string())
        .bind(personal_color)
        .fetch_optional(pool)
        .await?;
        return Ok(user_id);
    }
    Ok(None)
}

pub async fn change_username(
    pool: &SqlitePool, 
    user_id: u32, 
    new_username: &str
    ) -> Result<String, sqlx::Error> {
    let old_username: String = sqlx::query_scalar("
        WITH target AS (
            SELECT id, username AS old_username
            FROM User
            WHERE id = ?
        )
        UPDATE User
        SET username = ?
        WHERE id = (SELECT id FROM target)
        RETURNING (SELECT old_username FROM target) AS old_username;
        ").bind(user_id).bind(new_username).fetch_one(pool).await?;
    Ok(old_username)
}
