import type { PageServerLoad, Actions } from "./$types.js";
import { superValidate, fail, message, setError } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { bookFormSchema } from "./book-form-schema.js";
import { parseDate } from 'chrono-node';
import { error, redirect } from "@sveltejs/kit";
import { backendPOST } from "$lib/utils-server.js";
import imageCompression from "browser-image-compression";

const GOOGLE_BOOKS_API_URL = "https://www.googleapis.com/books/v1/volumes?q=isbn:";

export const load: PageServerLoad = async ({ url }) => {
  let form = await superValidate(zod(bookFormSchema))

  const isbn = url.searchParams.get("isbn")

  let coverURL = "";

  if (isbn != null) {
    let book = await fetchBook(isbn)
    form.data.isbn = isbn
    if (book != null) {
        form.data.title = book.title
        form.data.authors = (book.authors as string[]).join("\n")
        let date = parseDate(book.publishedDate ?? "")
        if (date != null) {
            form.data.publication_year = date.getFullYear()
        } else {
            form.errors.publication_year = [`Could not parse year from ${book.publishedDate}`]
        }
        form.data.page_count = book.pageCount
        form.data.language = book.language
        // Only assign genres to categories if field exists
        book.categories && (form.data.genres = (book.categories as string[]).join("\n"));
        
        coverURL = book.imageLinks?.thumbnail;
    } else {
        // Custom error message to show under isbn field
        form.errors.isbn = [`Could not find book with ISBN ${isbn}`]
    }
  }
  
  return {
    form: form,
    coverURL
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
        genres: bookForm.data.genres,
        publication_year: bookForm.data.publication_year,
        page_count: bookForm.data.page_count,
        language: bookForm.data.language,
    }

    const coverURL = await fetchBook(book.isbn).then(b => b.imageLinks?.thumbnail);

    const cover = await fetchBookCover(coverURL);

    const formData = new FormData();

    formData.append("json", new Blob([JSON.stringify(book)], { type: "application/json" }))
    if (bookForm.data.cover) {
        formData.append("file", bookForm.data.cover);
    } else if (cover) {
        formData.append("file", cover);
    }

    let response = await backendPOST(event.cookies, "/register_book", formData);

    if (response.status >= 400) {
        return setError(bookForm, response.data)
    }

    redirect(303, "/book/" + bookForm.data.isbn);
  },
};

async function fetchBook(isbn: string | null) {
    let resp = await fetch(GOOGLE_BOOKS_API_URL + isbn).then((data) => data.json())
    return resp?.items?.[0]?.volumeInfo
}

async function fetchBookCover(url: string) {
    try {
        const blob = await fetch(url).then(resp => resp.blob());
        return new File([blob], `cover.${extensions[blob.type]}`, { type: blob.type });
    } catch {
        return undefined
    }
}

const extensions: Record<string, string> = {
  "image/jpeg": "jpg",
  "image/png": "png",
  "image/webp": "webp"
};
