import type { PageServerLoad } from './$types';

import { error } from "@sveltejs/kit";

import placeHolderImage from "$lib/assets/placeholder_image.webp";
import { BACKEND_URL } from '$lib/utils-server';

export let load: PageServerLoad = async ({ fetch, params, url }) => {
    let bookResponse = await fetch(BACKEND_URL + "/book/" + params.identifier);

    if (!bookResponse.ok) {
        console.log(await bookResponse.text())
        throw error(bookResponse.status)
    }

    let book = await bookResponse.json();
    let coverImage = BACKEND_URL + "/book_cover/" + book.book.uuid + ".webp";
    coverImage = await fetch(coverImage, { method: "HEAD" })
        .then(res => res.ok ? coverImage : placeHolderImage)
        .catch(_ => placeHolderImage)

    let shelvesString = await fetch(BACKEND_URL + "/get_shelves")
        .then(resp => resp.text())
        .catch(_ => "");

    return {
        book: book.book,
        copies: book.copies,
        cover: coverImage,
        shelves: shelvesString === "" ? [] : shelvesString.split(","),
        copy: url.searchParams.get("copy")

    }
};
