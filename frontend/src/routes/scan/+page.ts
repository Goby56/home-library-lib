import type { PageLoad } from './$types';
import { redirect } from '@sveltejs/kit';

const QRBOT_SCAN_URL = "https://qrbot.net/x-callback-url/scan?x-success=";
const GOOGLE_BOOKS_API_URL = "https://www.googleapis.com/books/v1/volumes?q=isbn:";
const SELF_URL = "http://192.168.1.223:5173/scan";

export const load: PageLoad = async ({ url }) => {
  const shouldRedirect = url.searchParams.get("redirect");

  // qrbot reference
  // https://ioshelp.qrbot.net/xcallback
  
  if (shouldRedirect === "true") {
    throw redirect(302, QRBOT_SCAN_URL + SELF_URL);
  }

  const source = url.searchParams.get("x-source")
  const content = url.searchParams.get("content")
  const format = url.searchParams.get("format")
  
  let book = null
  if (source === "qrbot") {
    if (format === "ean13") {
        book = await fetchBook(content)
    }
  }

  const isbn = url.searchParams.get("isbn")
  if (isbn != null) {
    book = await fetchBook(isbn)
  }

  return {
      source: source,
      content: content,
      format: format,
      book: book,
  };
};

async function fetchBook(isbn: string | null) {
    let resp = await fetch(GOOGLE_BOOKS_API_URL + isbn).then((data) => data.json())
    if (typeof resp?.items !== "undefined") {
        return resp.items[0].volumeInfo
    }
}

// const GOOGLE_BOOKS_API_URL: &str = "https://www.googleapis.com/books/v1/volumes?q=isbn:";
// 
// #[derive(Debug, Deserialize)]
// struct ApiResponse {
//     items: Vec<BookItem>
// }
// 
// #[derive(Debug, Deserialize)]
// struct BookItem {
//     volumeInfo: VolumeInfo,
// }
// 
// #[derive(Debug, Deserialize)]
// struct VolumeInfo {
//     title: String,
//     authors: Vec<String>,
//     publishedDate: String,
//     language: String,
//     pageCount: u16,
//     categories: Vec<String>
// }
