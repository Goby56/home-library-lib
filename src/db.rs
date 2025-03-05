use sqlx::SqlitePool;

struct Book {
    title: String,
    author: String,
    publication_date: String,
    genre: String,
    page_count: u16,
    language: String,
    isbn: String,
}

async fn insert_book(pool: &SqlitePool, book: Book) -> Result<(), sqlx::Error> {
    // Insert the book
    let book_id = sqlx::query(
        "INSERT INTO books (title, author, genre, publication_date, isbn, page_count, language) 
         VALUES (?, ?, ?, ?, ?, ?, ?) RETURNING id")
        .bind(book.title)
        .bind(book.author)
        .bind(book.genre)
        .bind(book.publication_date)
        .bind(book.isbn)
        .bind(book.page_count)
        .bind(book.language)
        .fetch_one(pool);

    // Split title and author into individual words
    // let title_words: Vec<&str> = title.split_whitespace().collect();
    // let author_words: Vec<&str> = author.split_whitespace().collect();

    // Insert each word into spellfix1
    // for word in title_words.iter().chain(author_words.iter()).chain(std::iter::once(&genre)) {
    //     sqlx::query!(
    //         "INSERT INTO books_spellfix (word) SELECT ? WHERE NOT EXISTS (SELECT 1 FROM books_spellfix WHERE word = ?)",
    //         word, word
    //     ).execute(pool).await?;
    // }

    Ok(())
}

