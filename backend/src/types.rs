
use serde::Serialize;
use time::PrimitiveDateTime;

pub trait Item {
    fn item_type() -> ItemType;

    fn data<T: Serialize>() -> T;
    
    fn status() -> ReservationStatus;
}

pub enum ItemType {
    Book
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ReservationStatus {
    id: u32,
    user: String,
    timestamp: PrimitiveDateTime,
    start_date:  PrimitiveDateTime,
    end_date: Option<PrimitiveDateTime>,
}

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct Shelf {
    pub id: u32,
    pub name: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Book {
    pub id: u32,
    pub isbn: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publication_year: i16,
    pub genres: Vec<String>,
    pub page_count: u16,
    pub language: String,
    pub copies: Vec<u32>
}

#[derive(serde::Serialize)]
pub struct PhysicalBook {
    pub id: u32,
    pub shelf: Shelf,
    pub reservation: Option<ReservationStatus>
}
