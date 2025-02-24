use isbn::Isbn;

const GOOGLE_BOOKS_API_URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn:";

pub fn get_book_metadata(isbn: Isbn) -> Result<serde_json::Value, reqwest::Error> {
    match reqwest::blocking::get(GOOGLE_BOOKS_API_URL.to_owned() + &isbn.to_string()) {
        Ok(resp) => {
            let json: serde_json::Value = resp.json()?;
            println!("{:?}", json);
            return Ok(json);
        },
        Err(err) => Err(err)
    }
}
