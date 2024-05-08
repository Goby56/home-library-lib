use isbn::Isbn;

pub struct Book {
    title: String,
    author: String,
    pub_date: u16,
    isbn: Isbn
}

pub trait Searchable {
    fn search_str(&self) -> String;
}

impl Searchable for Book {
    fn search_str(&self) -> String {
        return format!("{} | {}", self.title, self.author);
    }
}
