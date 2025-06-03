CREATE TABLE "Author" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "Book" (
	"id"	INTEGER NOT NULL UNIQUE,
	"isbn"	TEXT NOT NULL UNIQUE,
	"title"	TEXT NOT NULL,
	"publication_year"	INTEGER NOT NULL,
	"page_count"	INTEGER NOT NULL,
	"language"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "PhysicalBook" (
	"id"	INTEGER NOT NULL UNIQUE,
    "book" INTEGER NOT NULL,
	"shelf"	INTEGER NOT NULL,
	"reservation"	INTEGER,
	FOREIGN KEY("book") REFERENCES "Book"("id"),
	FOREIGN KEY("reservation") REFERENCES "Reservation"("id"),
	FOREIGN KEY("shelf") REFERENCES "Shelf"("id")
);
CREATE TABLE "BookContribution" (
	"book"	INTEGER,
	"author"	INTEGER,
	UNIQUE("book","author"),
	FOREIGN KEY("author") REFERENCES "Author"("id"),
	FOREIGN KEY("book") REFERENCES "Book"("id")
);
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
CREATE TABLE "Reservation" (
	"id"	INTEGER NOT NULL UNIQUE,
	"user"	INTEGER NOT NULL,
	"timestamp"	TEXT DEFAULT CURRENT_TIMESTAMP,
    "start_date" TEXT NOT NULL,
    "end_date" TEXT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "Shelf" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "User" (
    "id"    INTEGER NOT NULL UNIQUE,
    "name"  TEXT NOT NULL UNIQUE,
    "password" TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
