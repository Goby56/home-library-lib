use sqlx::{pool, SqlitePool};

use crate::types;

pub async fn insert_book(pool: &SqlitePool, book: types::Book) -> Result<(), sqlx::Error> {
    let book_id = sqlx::query_as::<_, (i64,)>("
        INSERT INTO Book (isbn, shelf, reservation, title, publication_year, page_count, language)
        VALUES (?, NULL, NULL, ?, ?, ?, ?) RETURNING id")
        .bind(book.isbn)
        .bind(book.title)
        .bind(book.publication_year)
        .bind(book.page_count)
        .bind(book.language)
        .fetch_one(pool).await?;
    
    for author in book.authors { // Insert new author and connect to book
        let author_id: Option<(i64,)> = sqlx::query_as("
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

    // TODO Try better API that actually provides genre
    for genre in book.genres { // Insert genre and genre connection to book
        let genre_id: Option<(i64,)> = sqlx::query_as("
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
    Ok(())
}

// pub async fn _insert_book(pool: &SqlitePool, book: Book) -> Result<(), sqlx::Error> {
//     // Insert the book
//     sqlx::query(
//         "INSERT INTO books (title, author, genre, publication_date, isbn, page_count, language) 
//          VALUES (?, ?, ?, ?, ?, ?, ?)")
//         .bind(book.title.clone())
//         .bind(book.authors.join(", "))
//         .bind(book.genres.join(", "))
//         .bind(book.publication_date)
//         .bind(book.isbn)
//         .bind(book.pages)
//         .bind(book.language)
//         .execute(pool).await?;
//     
//     let title_words = book.title.split_whitespace();
//     let authors_string = book.authors.join(" ");
// 
//     // Insert individual words from title and author names into spellfix
//     for word in title_words.chain(authors_string.split_whitespace()) {
//         sqlx::query(
//             "INSERT INTO books_spellfix (word) SELECT ? WHERE NOT EXISTS 
//             (SELECT 1 FROM books_spellfix WHERE word = ?)")
//             .bind(word)
//             .bind(word).execute(pool).await?;
//     }
//     Ok(())
// }
//

pub async fn query_books(pool: &SqlitePool, _search_str: Option<String>, isbn: Option<String>, limit: Option<u32>) -> Result<Vec<types::Book>, sqlx::Error>{
    let mut sq = String::from(r#"
        SELECT 
            Book.title,
            Book.isbn,
            GROUP_CONCAT(Author.name, ',') AS authors,
            Book.publication_year,
            GROUP_CONCAT(Genre.name, ',') AS genres,
            Book.page_count,
            Book.language
        FROM Book
        JOIN BookContribution ON Book.id = BookContribution.book
        JOIN Author ON BookContribution.author = Author.id
        JOIN GenreMatch ON Book.id = GenreMatch.book
        JOIN Genre ON GenreMatch.genre = Genre.id
        "#);

    if isbn.is_some() {
        sq.push_str(" WHERE Book.isbn = ?");
    }

    sq.push_str(" GROUP BY Book.id, Book.title");

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
    let books: Vec<(String, String, String, i16, String, u16, String)> = query.fetch_all(pool).await?;

    return Ok(books.into_iter().map(|b| {
        types::Book {
            title: b.0,
            isbn: b.1,
            authors: b.2.split(",").map(|s| s.to_string()).collect(),
            publication_year: b.3,
            genres: b.4.split(",").map(|s| s.to_string()).collect(),
            page_count: b.5,
            language: b.6,
    }

    }).collect())
}
