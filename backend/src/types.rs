
// use serde::Serialize;
use time::Date;

// pub trait Item {
//     fn item_type() -> ItemType;
// 
//     fn data<T: Serialize>() -> T;
//     
//     fn status() -> ReservationStatus;
// }

// pub enum ItemType {
//     Book
// }

#[derive(sqlx::FromRow, serde::Serialize)]
pub struct ReservationStatus {
    pub id: u32,
    pub user: u32,
    pub created_at: i64,
    pub start_date:  Date,
    pub end_date: Option<Date>,
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
    pub copy_ids: Vec<u32>
}

#[derive(serde::Serialize)]
pub struct PhysicalBook {
    pub id: u32,
    pub shelf: Shelf,
    pub reservation: Option<ReservationStatus>,
}
