#![allow(non_snake_case)]

use serde::Deserialize;

use crate::database::Book;

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
    language: String,
    pageCount: u16,
    categories: Vec<String>
}

pub async fn fetch_book_metadata(isbn: &str) -> Vec<Book> {
    match fetch_from_google_books_api(isbn).await {
        Ok(resp) => {
            let mut books = vec![];
            for book in resp.items {
                books.push(Book {
                    title: book.volumeInfo.title,
                    authors: book.volumeInfo.authors,
                    publication_date: book.volumeInfo.publishedDate,
                    language: book.volumeInfo.language,
                    pages: book.volumeInfo.pageCount,
                    genres: book.volumeInfo.categories,
                    isbn: isbn.to_string()
                })
            }
            return books;
            
        },
        Err(_) => vec![]
    }
}

async fn fetch_from_google_books_api(isbn: &str) -> Result<ApiResponse, reqwest::Error> {
    match reqwest::get(GOOGLE_BOOKS_API_URL.to_owned() + isbn).await {
        Ok(resp) => resp.json::<ApiResponse>().await,
        Err(err) => Err(err)
    }
}
