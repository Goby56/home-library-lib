use sqlx::SqlitePool;

use crate::types;

pub async fn get_book_from_isbn(pool: &SqlitePool, isbn: String) -> Result<(Option<types::Book>, Vec<types::PhysicalBook>), sqlx::Error>  {
    let book = get_books(pool, None, Some(isbn), Some(1)).await?.pop(); 
    // Vec should be length 0 or 1 so pop will give that element
                                                        
    let mut physical_copies = vec![];
    if let Some(b) = &book {
        for copy_id in &b.copies {
            if let Some(copy) = get_physical_book(pool, *copy_id).await? {
                physical_copies.push(copy); 
            }
        }
    }
    Ok((book, physical_copies))
}

async fn get_physical_book(pool: &SqlitePool, id: u32) -> Result<Option<types::PhysicalBook>, sqlx::Error> {
    let copy: Option<(u32, u32)> = sqlx::query_as("
        SELECT shelf, reservation
        FROM PhysicalBook 
        WHERE id = ?").bind(id).fetch_optional(pool).await?;
    if copy.is_none() {
        return Ok(None);
    }
    let shelf = get_shelf(pool, copy.unwrap().0).await?;
    let reservation = get_reservation(pool, copy.unwrap().1).await?;
    if shelf.is_none() {
        return Ok(None);
    }
    Ok(Some(types::PhysicalBook {
        id, shelf: shelf.unwrap(), reservation
    }))
}

pub async fn get_shelf(pool: &SqlitePool, id: u32) -> Result<Option<types::Shelf>, sqlx::Error> {
   let shelf: Option<types::Shelf> = sqlx::query_as("
       SELECT id, name 
       FROM Shelf 
       WHERE id = ?").bind(id).fetch_optional(pool).await?;
   Ok(shelf)
}

pub async fn get_reservation(pool: &SqlitePool, id: u32) -> Result<Option<types::ReservationStatus>, sqlx::Error> {
   let shelf: Option<types::ReservationStatus> = sqlx::query_as(r#"
       SELECT 
           Reservation.id, 
           User.name, 
           Reservation.timestamp, 
           Reservation.start_date, 
           Reservation.end_date
       FROM Reservation
       JOIN User ON User.id = Reservation.user
       WHERE Reservation.id = ?"#).bind(id).fetch_optional(pool).await?;
   Ok(shelf)
}

pub async fn create_physical_book(pool: &SqlitePool, book: u32, shelf: u32) -> Result<(), sqlx::Error> {
    sqlx::query("
        INSERT INTO PhysicalBook (book, shelf, reservation)
        VALUES (?, ?, NULL)")
        .bind(book).bind(shelf).execute(pool).await?;
    Ok(())
}

pub async fn insert_book(pool: &SqlitePool, book: types::Book) -> Result<u32, sqlx::Error> {
    let book_id = sqlx::query_as::<_, (u32,)>("
        INSERT INTO Book (isbn, title, publication_year, page_count, language)
        VALUES (?, ?, ?, ?, ?) RETURNING id")
        .bind(book.isbn)
        .bind(book.title)
        .bind(book.publication_year)
        .bind(book.page_count)
        .bind(book.language)
        .fetch_one(pool).await?;
    
    for author in book.authors { // Insert new author and connect to book
        let author_id: Option<(u32,)> = sqlx::query_as("
            INSERT INTO Author (name) VALUES (?)
            ON CONFLICT(name) DO UPDATE SET name=name
            RETURNING id")
            .bind(author)
            .fetch_optional(pool).await?;
        if let Some(id) = author_id {
            sqlx::query("
                INSERT INTO BookContribution (book, author) VALUES (?, ?)")
                .bind(book_id.0)
                .bind(id.0)
                .execute(pool).await?;
        }
    }

    for genre in book.genres { // Insert genre and genre connection to book
        let genre_id: Option<(u32,)> = sqlx::query_as("
            INSERT INTO Genre (name) VALUES (?)
            ON CONFLICT(name) DO UPDATE SET name=name
            RETURNING id")
            .bind(genre)
            .fetch_optional(pool).await?;
        if let Some(id) = genre_id {
            sqlx::query("
                INSERT INTO GenreMatch (book, genre) VALUES (?, ?)")
                .bind(book_id.0)
                .bind(id.0)
                .execute(pool).await?;
        }
    }
    Ok(book_id.0)
}

pub async fn get_books(pool: &SqlitePool, _search_str: Option<String>, isbn: Option<String>, limit: Option<u32>) -> Result<Vec<types::Book>, sqlx::Error>{
    let mut sq = String::from(r#"
        SELECT 
            Book.title,
            Book.isbn,
            GROUP_CONCAT(DISTINCT Author.name, ',') AS authors,
            Book.publication_year,
            GROUP_CONCAT(DISTINCT Genre.name, ',') AS genres,
            Book.page_count,
            Book.language,
            GROUP_CONCAT(DISTINCT PhysicalBook.id, ',') as copies,
        FROM Book
        JOIN PhysicalBook ON Book.id = PhysicalBook.book
        JOIN BookContribution ON Book.id = BookContribution.book
        JOIN Author ON BookContribution.author = Author.id
        JOIN GenreMatch ON Book.id = GenreMatch.book
        JOIN Genre ON GenreMatch.genre = Genre.id
        "#);

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
    let books: Vec<(String, String, String, i16, String, u16, String, String)> = query.fetch_all(pool).await?;

    return Ok(books.into_iter().map(|b| {
        types::Book {
            title: b.0,
            isbn: b.1,
            authors: b.2.split(",").map(|s| s.to_string()).collect(),
            publication_year: b.3,
            genres: b.4.split(",").map(|s| s.to_string()).collect(),
            page_count: b.5,
            language: b.6,
            copies: b.7.split(",").map(|s| s.parse::<u32>().unwrap()).collect(),
    }

    }).collect())
}
