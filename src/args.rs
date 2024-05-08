use clap::{Parser, Subcommand, Args};

/// HLL - Home library (lib) : A CLI to store, search and lend books
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: LibraryInteraction,
}

#[derive(Debug, Subcommand)]
pub enum LibraryInteraction {
    /// Add a new book to the library
    Shelve(ShelveCommand),
    /// See where a book is located. Search by its title, author, publish date or ISBN
    Search(SearchCommand),
    /// Borrow a book
    Borrow(BorrowCommand),
    /// Return a book that you have borrowed
    Return(ReturnCommand),
}

#[derive(Debug, Args)]
pub struct  ShelveCommand {
    /// Book title
    pub title: String,
    /// Book author
    pub author: String,
    /// Publish date
    pub publish_date: i16,
    /// ISBN
    pub isbn: String,
}

#[derive(Debug, Args)]
pub struct SearchCommand {
    /// Title, author, publish date or ISBN
    pub search_str: String
}

#[derive(Debug, Args)]
pub struct BorrowCommand {
    /// ISBN of the book you want to borrow
    pub isbn: String
}

#[derive(Debug, Args)]
pub struct ReturnCommand {
    /// ISBN of the book you want to return
    pub isbn: String
}
