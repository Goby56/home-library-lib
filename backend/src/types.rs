
// use serde::Serialize;
use time::OffsetDateTime;

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

#[derive(sqlx::FromRow, serde::Serialize, Clone)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub personal_color: String,
}

#[derive(serde::Serialize)]
pub struct Reservation {
    pub id: u32,
    pub user: User,
    pub created_at: i64,
    #[serde(with = "time::serde::iso8601")]
    pub start_date:  OffsetDateTime,
    #[serde(with = "time::serde::iso8601")]
    pub end_date: OffsetDateTime,
}

impl Reservation {
    pub fn intersects(&self, start: OffsetDateTime, end: OffsetDateTime) -> bool {
        // Reservations can start and end on the same date
        self.start_date < end && start < self.end_date 
    }
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
    pub reservations: Vec<Reservation>,
}
