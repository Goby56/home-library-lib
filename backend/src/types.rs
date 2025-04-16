use serde::Serialize;

pub trait Item {
    fn item_type() -> ItemType;

    fn data<T: Serialize>() -> T;
    
    fn status() -> ReservationStatus;
}

pub enum ItemType {
    Book
}

#[derive(serde::Serialize)]
pub struct ReservationStatus {
   start_date:  String,
   end_date: Option<String>,
   borrower: String,
}

#[derive(serde::Serialize)]
pub struct Book {
    pub title: String,
    pub authors: Vec<String>,
    pub publication_date: String,
    pub genres: Vec<String>,
    pub pages: u16,
    pub language: String,
    pub isbn: String,
}
