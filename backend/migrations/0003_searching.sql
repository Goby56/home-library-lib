CREATE VIRTUAL TABLE "BookFts" USING fts5 (book_id, title, authors, genres);

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
