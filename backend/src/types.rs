
use time::OffsetDateTime;
use uuid::Uuid;

// pub trait Location {
// 
// }
// 
// pub trait Item {
//     fn item_type() -> ItemType;
// 
//     fn repr() -> String;
// 
//     fn location() -> impl Location;
// 
//     fn data<T: serde::Serialize>() -> T;
// }
// 
// pub enum ItemType {
//     Book(Book, PhysicalBook)
// }
// 
// pub struct GenericItem {
//     item_type: ItemType,
//     name: String,
//     location: String,
//     reservation: Reservation,
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
    pub uuid: Uuid,
    pub isbn: Option<String>,
    pub title: String,
    pub authors: Vec<String>,
    pub genres: Vec<String>,
    pub publication_year: Option<i16>,
    pub page_count: Option<u16>,
    pub language: Option<String>,
    pub copy_ids: Vec<u32>
}

#[derive(serde::Serialize)]
pub struct PhysicalBook {
    pub id: u32,
    pub shelf: Shelf,
    pub reservations: Vec<Reservation>,
}
