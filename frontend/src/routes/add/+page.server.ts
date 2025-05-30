import type { PageServerLoad, Actions } from "./$types.js";
import { fail } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { bookFormSchema } from "./book-form-schema";
import { parseDate } from 'chrono-node';
import axios from "axios";
import placeHolderImage from "$lib/assets/placeholder_image.webp";

const GOOGLE_BOOKS_API_URL = "https://www.googleapis.com/books/v1/volumes?q=isbn:";

export const load: PageServerLoad = async ({ url }) => {
  let form = await superValidate(zod(bookFormSchema))

  const isbn = url.searchParams.get("isbn")

  if (isbn != null) {
    let book = await fetchBook(isbn)
    form.data.isbn = isbn
    if (book != null) {
        form.data.title = book.title
        form.data.authors = book.authors
        let date = parseDate(book.publishedDate ?? "")
        if (date != null) {
            form.data.publication_year = date.getFullYear()
        } else {
            form.errors.publication_year = [`Could not parse year from ${book.publishedDate}`]
        }
        form.data.page_count = book.pageCount
        form.data.language = book.language
        // Only assign genres to categories if field exists
        book.categories && (form.data.genres = book.categories);
    } else {
        // Custom error message to show under isbn field
        form.errors.isbn = [`Could not find book with ISBN ${isbn}`]
    }
  }
  
  return {
    form: form
  };
};

export const actions: Actions = {
  default: async (event) => {
    const bookForm = await superValidate(event, zod(bookFormSchema));
    if (!bookForm.valid) {
      return fail(400, {
        form: bookForm,
      });
    }

    let book = {
        isbn: bookForm.data.isbn,
        title: bookForm.data.title,
        authors: bookForm.data.authors,
        publication_year: bookForm.data.publication_year,
        language: bookForm.data.language,
        page_count: bookForm.data.page_count,
        genres: bookForm.data.genres,
    }

    const formData = new FormData();

    formData.append("json", new Blob([JSON.stringify(book)], { type: "application/json" }))

    formData.append("file", bookForm.data.cover[0])

    
    let response = {
        success: false,
        message: ""
    }
        
    await axios.post("http://192.168.1.223:8080/shelve", bookForm.data)
        .then(resp => {
            response.success = true;
            response.message = resp.data;
        })
        .catch(err => {
            response.success = false;
            response.message = err;
        })
    return {
      form: bookForm,
      response,
    };
  },
};

async function fetchBook(isbn: string | null) {
    let resp = await fetch(GOOGLE_BOOKS_API_URL + isbn).then((data) => data.json())
    return resp?.items?.[0]?.volumeInfo
}

