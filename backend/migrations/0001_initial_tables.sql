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
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("book") REFERENCES "Book"("id"),
	FOREIGN KEY("shelf") REFERENCES "Shelf"("id")
);
CREATE TABLE "BookReservationMatch" (
    "physical_book" INTEGER,
    "reservation" INTEGER,
    UNIQUE("physical_book", "reservation"),
	FOREIGN KEY("physical_book") REFERENCES "PhysicalBook"("id") ON DELETE CASCADE,
	FOREIGN KEY("reservation") REFERENCES "Reservation"("id") ON DELETE CASCADE
);
CREATE TABLE "BookContribution" (
	"book"	INTEGER,
	"author"	INTEGER,
	UNIQUE("book","author"),
	FOREIGN KEY("author") REFERENCES "Author"("id") ON DELETE CASCADE,
	FOREIGN KEY("book") REFERENCES "Book"("id") ON DELETE CASCADE
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
	FOREIGN KEY("genre") REFERENCES "Genre"("id") ON DELETE CASCADE,
	FOREIGN KEY("book") REFERENCES "Book"("id") ON DELETE CASCADE
);
CREATE TABLE "Reservation" (
	"id"	INTEGER NOT NULL UNIQUE,
	"user"	INTEGER NOT NULL,
	"created_at"	INTEGER NOT NULL,
    "start_date" TEXT NOT NULL,
    "end_date" TEXT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
    FOREIGN KEY("user") REFERENCES "User"("id") ON DELETE CASCADE
);
CREATE TABLE "Shelf" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "User" (
    "id"    INTEGER NOT NULL UNIQUE,
    "username"  TEXT NOT NULL UNIQUE,
    "password_hash" TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
