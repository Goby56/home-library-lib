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

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Book {
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publication_year: i16,
    pub genres: Vec<String>,
    pub page_count: u16,
    pub language: String,
}
