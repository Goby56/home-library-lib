CREATE TABLE `Book`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `isbn` INT NOT NULL,
    `shelf` BIGINT NULL,
    `borrow` BIGINT NULL,
    `title` TEXT NOT NULL,
    `publication_date` DATE NOT NULL,
    `pages` INT NOT NULL,
    `language` TEXT NOT NULL
);
CREATE TABLE `Borrow`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `borrower` BIGINT NOT NULL,
    `date` DATETIME NOT NULL
);
CREATE TABLE `Shelf`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL
);
CREATE TABLE `Author`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL UNIQUE
);
CREATE TABLE `Genre`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `name` TEXT NOT NULL UNIQUE
);
CREATE TABLE `BookContribution`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `book` BIGINT NOT NULL,
    `author` BIGINT NOT NULL
);
CREATE TABLE `GenreMatch`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `book` BIGINT NOT NULL,
    `genre` BIGINT NOT NULL
);
CREATE TABLE `User`(
    `id` BIGINT UNSIGNED NOT NULL PRIMARY KEY,
    `name` BIGINT NOT NULL,
    `password` BIGINT NOT NULL
);
ALTER TABLE
    `Book` ADD CONSTRAINT `book_shelf_foreign` FOREIGN KEY(`shelf`) REFERENCES `Shelf`(`id`);
ALTER TABLE
    `GenreMatch` ADD CONSTRAINT `genrematch_genre_foreign` FOREIGN KEY(`genre`) REFERENCES `Genre`(`id`);
ALTER TABLE
    `BookContribution` ADD CONSTRAINT `bookcontribution_book_foreign` FOREIGN KEY(`book`) REFERENCES `Book`(`id`);
ALTER TABLE
    `Book` ADD CONSTRAINT `book_borrow_foreign` FOREIGN KEY(`borrow`) REFERENCES `Borrow`(`id`);
ALTER TABLE
    `BookContribution` ADD CONSTRAINT `bookcontribution_author_foreign` FOREIGN KEY(`author`) REFERENCES `Author`(`id`);
ALTER TABLE
    `GenreMatch` ADD CONSTRAINT `genrematch_book_foreign` FOREIGN KEY(`book`) REFERENCES `Book`(`id`);
ALTER TABLE
    `Borrow` ADD CONSTRAINT `borrow_borrower_foreign` FOREIGN KEY(`borrower`) REFERENCES `User`(`id`);
