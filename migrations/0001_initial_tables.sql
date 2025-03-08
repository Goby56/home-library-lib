CREATE TABLE "Author" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "Book" (
	"id"	INTEGER NOT NULL UNIQUE,
	"isbn"	TEXT NOT NULL,
	"shelf"	INTEGER,
	"borrow"	INTEGER,
	"title"	TEXT NOT NULL,
	"publication_date"	TEXT NOT NULL,
	"pages"	INTEGER NOT NULL,
	"language"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("borrow") REFERENCES "Borrow"("id"),
	FOREIGN KEY("shelf") REFERENCES "Shelf"("id")
);
CREATE TABLE "BookContribution" (
	"book"	INTEGER,
	"author"	INTEGER,
	UNIQUE("book","author"),
	FOREIGN KEY("author") REFERENCES "Author"("id"),
	FOREIGN KEY("book") REFERENCES "Book"("id")
);
CREATE TABLE "Borrow" (
	"id"	INTEGER NOT NULL UNIQUE,
	"borrower"	INTEGER,
	"timestamp"	TEXT,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE "Shelf" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	INTEGER NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
