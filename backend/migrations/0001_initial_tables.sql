CREATE TABLE "Author" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "Book" (
	"id"	INTEGER NOT NULL UNIQUE,
	"isbn"	TEXT NOT NULL,
	"shelf"	INTEGER,
	"reservation"	INTEGER,
	"title"	TEXT NOT NULL,
	"publication_year"	INTEGER NOT NULL,
	"page_count"	INTEGER NOT NULL,
	"language"	TEXT NOT NULL,
    "date_added" TEXT DEFAULT CURRENT_TIMESTAMP,
	PRIMARY KEY("id" AUTOINCREMENT),
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
CREATE TABLE "Reservation" (
	"id"	INTEGER NOT NULL UNIQUE,
	"user"	INTEGER,
	"timestamp"	TEXT DEFAULT CURRENT_TIMESTAMP,
    "start_Date" TEXT NOT NULL,
    "end_date" TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "Shelf" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	INTEGER NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
