import type { PageServerLoad, Actions } from "./$types.js";
import { superValidate, fail, message, setError } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { bookFormSchema } from "$lib/components/book-form/book-form-schema.js";
import { parseDate } from 'chrono-node';
import { error, redirect } from "@sveltejs/kit";
import { BACKEND_URL, backendPOST } from "$lib/utils-server.js";
import imageCompression from "browser-image-compression";

async function getBook(fetch: (input: string) => Promise<Response>, identifier: string) {
    let bookResponse = await fetch(BACKEND_URL + "/book/" + identifier);

    if (!bookResponse.ok) {
        console.log(await bookResponse.text())
        throw error(bookResponse.status)
    }
    
    return (await bookResponse.json()).book;
}

export const load: PageServerLoad = async ({ fetch, params }) => {
    let form = await superValidate(zod(bookFormSchema))

    let book = await getBook(fetch, params.identifier);

    form.data.isbn = book.isbn;
    form.data.title = book.title;
    form.data.authors = book.authors?.join("\n") ?? "";
    form.data.genres = book.genres?.join("\n") ?? "";
    form.data.publication_year = book.publication_year;
    form.data.page_count = book.page_count;
    form.data.language = book.language;
    
    return {
      form: form,
      coverURL: BACKEND_URL + "/book_cover/" + book.uuid + ".webp"
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

    let oldBook = await getBook(event.fetch, event.params.identifier);
    
    let book = {
        isbn: bookForm.data.isbn,
        title: bookForm.data.title,
        authors: bookForm.data.authors,
        genres: bookForm.data.genres,
        publication_year: bookForm.data.publication_year,
        page_count: bookForm.data.page_count,
        language: bookForm.data.language,
    }

    const formData = new FormData();

    formData.append("book", new Blob([JSON.stringify(book)], { type: "application/json" }))
    if (bookForm.data.cover) {
        formData.append("cover", bookForm.data.cover);
    }

    let response = await backendPOST(event.cookies, "/edit_book/" + oldBook.uuid, formData);

    if (response.status >= 400) {
        return setError(bookForm, response.data)
    }
    
    redirect(303, "/book/" + response.data);
  },
};
