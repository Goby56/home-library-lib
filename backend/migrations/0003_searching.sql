-- Full-text search table
CREATE VIRTUAL TABLE "BookFts" USING fts5 (
    book_id UNINDEXED, 
    title, 
    authors, 
    genres, 
    tokenize = "unicode61 remove_diacritics 0"
);

-- Automatic updates of fts table
CREATE TRIGGER "InsertBookTrigger" 
    AFTER INSERT ON "Book"
BEGIN
    INSERT INTO "BookFts" (book_id, title, authors, genres)
    VALUES (NEW.id, NEW.title, NEW.authors, NEW.genres);
END;

CREATE TRIGGER "UpdateBookTrigger" 
    AFTER UPDATE ON "Book"
BEGIN
    UPDATE "BookFts"
    SET
        title = NEW.title,
        authors = NEW.authors,
        genres = NEW.genres
    WHERE book_id = NEW.id;
END;

CREATE TRIGGER "DeleteBookTrigger" 
    AFTER DELETE ON "Book"
BEGIN
    DELETE FROM "BookFts"
    WHERE book_id = NEW.id;
END;

-- Direct access to fts table using aux/vocab table
CREATE VIRTUAL TABLE "BookFtsVocab" USING fts5vocab("BookFts", "row");

-- Spellfix1 table
CREATE VIRTUAL TABLE "BookSpellfix" USING spellfix1;
