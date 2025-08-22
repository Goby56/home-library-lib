CREATE TABLE "Book" (
	"id"	INTEGER NOT NULL UNIQUE,
    "uuid" TEXT NOT NULL UNIQUE,
	"isbn"	TEXT UNIQUE,
	"title"	TEXT NOT NULL,
    "authors" TEXT NOT NULL, -- New lines separates
    "genres" TEXT, -- New lines separates
	"publication_year"	INTEGER,
	"page_count"	INTEGER,
	"language"	TEXT, -- Language code
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "PhysicalBook" (
	"id"	INTEGER NOT NULL UNIQUE,
    "book" INTEGER NOT NULL,
	"shelf"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("book") REFERENCES "Book"("id") ON DELETE CASCADE,
	FOREIGN KEY("shelf") REFERENCES "Shelf"("id")
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
CREATE TABLE "BookReservationMatch" (
    "physical_book" INTEGER,
    "reservation" INTEGER,
    UNIQUE("physical_book", "reservation"),
	FOREIGN KEY("physical_book") REFERENCES "PhysicalBook"("id") ON DELETE CASCADE,
	FOREIGN KEY("reservation") REFERENCES "Reservation"("id") ON DELETE CASCADE
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
    "personal_color" TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
