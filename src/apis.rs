#![allow(non_snake_case)]

use std::{str::FromStr, vec};

use chrono::NaiveDate;
use isbn::Isbn;
use serde::Deserialize;

use crate::storing::data::BookMetadata;

const GOOGLE_BOOKS_API_URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn:";

#[derive(Debug, Deserialize)]
struct ApiResponse {
    items: Vec<BookItem>
}

#[derive(Debug, Deserialize)]
struct BookItem {
    volumeInfo: VolumeInfo,
}

#[derive(Debug, Deserialize)]
struct VolumeInfo {
    title: String,
    authors: Vec<String>,
    publishedDate: String,
    language: Option<String>,
    pageCount: Option<u16>,
    categories: Vec<String>
}

pub async fn fetch_book_metadata(isbn: &Isbn) -> Vec<BookMetadata> {
    match fetch_from_google_books_api(isbn).await {
        Ok(resp) => {
            let mut books = vec![];
            for book in resp.items {
                books.push(BookMetadata {
                    title: book.volumeInfo.title,
                    authors: book.volumeInfo.authors,
                    pub_date: NaiveDate::from_str(&book.volumeInfo.publishedDate).ok(),
                    language: book.volumeInfo.language,
                    pages: book.volumeInfo.pageCount,
                    genres: book.volumeInfo.categories
                })
            }
            return books;
            
        },
        Err(_) => vec![]
    }
}

async fn fetch_from_google_books_api(isbn: &Isbn) -> Result<ApiResponse, reqwest::Error> {
    match reqwest::get(GOOGLE_BOOKS_API_URL.to_owned() + &isbn.to_string()).await {
        Ok(resp) => resp.json::<ApiResponse>().await,
        Err(err) => Err(err)
    }
}
