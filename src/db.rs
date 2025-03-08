use sqlx::SqlitePool;

pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub publication_date: String,
    pub genres: Vec<String>,
    pub pages: u16,
    pub language: String,
    pub isbn: String,
}

pub async fn insert_book(pool: &SqlitePool, book: Book) -> Result<(), sqlx::Error> {
    let book_id = sqlx::query_as::<_, (i64,)>("
        INSERT INTO Book (isbn, shelf, borrow, title, publication_date, pages, language)
        VALUES (?, NULL, NULL, ?, ?, ?, ?) RETURNING id")
        .bind(book.isbn)
        .bind(book.title)
        .bind(book.publication_date)
        .bind(book.pages)
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
    // for genre in book.genres { // Insert genre and genre connection to book
    //     let genre_id: Option<(i64,)> = sqlx::query_as("
    //         INSERT INTO Genre (name) VALUES (?)
    //         SELECT last_insert_rowid()")
    //         .bind(genre)
    //         .fetch_optional(pool).await?;
    //     if let Some(id) = genre_id {
    //         sqlx::query("
    //             INSERT INTO GenreMatch (book, genre) VALUES (?, ?)")
    //             .bind(book_id.0)
    //             .bind(id.0)
    //             .execute(pool).await?;
    //     }
    // }
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

pub async fn get_all_books(pool: &SqlitePool) -> Result<Vec<(String, String)>, sqlx::Error>{
    let books: Vec<(String, String)> = sqlx::query_as(
        "SELECT title, isbn FROM Book")
        .fetch_all(pool)
        .await?;
    return Ok(books);
}

