-- Add migration script here
-- Enable the required extensions (for SQLite, make sure they are compiled in)
PRAGMA foreign_keys = ON;

-- Create the main books table
CREATE TABLE books (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    author TEXT NOT NULL,
    genre TEXT NOT NULL,
    publication_date DATE,
    isbn TEXT UNIQUE,
    page_count INTEGER,
    language TEXT
);

-- Create the full-text search table (FTS5)
CREATE VIRTUAL TABLE books_fts USING fts5(
    title, 
    author, 
    genre,
    content='books', -- Links to the main table
    tokenize='trigram' -- Enables fuzzy search via trigrams
);

-- Automatically sync FTS table with books table
CREATE TRIGGER books_ai AFTER INSERT ON books BEGIN
    INSERT INTO books_fts (rowid, title, author, genre)
    VALUES (new.id, new.title, new.author, new.genre);
END;

CREATE TRIGGER books_ad AFTER DELETE ON books BEGIN
    DELETE FROM books_fts WHERE rowid = old.id;
END;

CREATE TRIGGER books_au AFTER UPDATE ON books BEGIN
    DELETE FROM books_fts WHERE rowid = old.id;
    INSERT INTO books_fts (rowid, title, author, genre)
    VALUES (new.id, new.title, new.author, new.genre);
END;

-- Create spellfix1 table for typo corrections
CREATE VIRTUAL TABLE books_spellfix USING spellfix1;

-- Populate spellfix1 with book titles, authors, and genres
INSERT INTO books_spellfix(word) 
SELECT DISTINCT title FROM books;
INSERT INTO books_spellfix(word) 
SELECT DISTINCT author FROM books;
INSERT INTO books_spellfix(word) 
SELECT DISTINCT genre FROM books;

-- Function to search with typo correction
CREATE VIEW books_spellfix_suggestions AS
SELECT word, top_correct FROM books_spellfix WHERE word MATCH '*';

-- Function to search books with fuzzy matching (trigram similarity)
CREATE VIEW books_fuzzy_search AS
SELECT books.id, books.title, books.author, books.genre,
       books.publication_date, books.isbn, books.page_count, books.language
FROM books_fts
WHERE books_fts MATCH '*'
ORDER BY rank;
