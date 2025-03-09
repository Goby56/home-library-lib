CREATE TABLE "Genre" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "GenreMatch" (
	"book"	INTEGER,
	"genre"	INTEGER,
	UNIQUE("book","genre"),
	FOREIGN KEY("genre") REFERENCES "Genre"("id"),
	FOREIGN KEY("book") REFERENCES "Book"("id")
);
