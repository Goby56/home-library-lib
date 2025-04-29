use sqlx::SqlitePool;

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

pub async fn get_all_books(pool: &SqlitePool) -> Result<Vec<types::Book>, sqlx::Error>{
    let books: Vec<(String, String, i16, String, u16)> = sqlx::query_as(
        "SELECT title, isbn, publication_year, language, page_count FROM Book")
        .fetch_all(pool)
        .await?;

    return Ok(books.iter().map(|b| {
        types::Book {
            title: b.0.clone(),
            isbn: b.1.clone(),
            authors: vec![],
            publication_year: b.2,
            genres: vec![],
            page_count: b.4,
            language: b.3.clone(),
        }
    }).collect())
}

