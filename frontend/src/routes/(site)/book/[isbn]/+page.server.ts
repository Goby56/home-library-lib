import type { PageServerLoad } from './$types';

import { error } from "@sveltejs/kit";

import placeHolderImage from "$lib/assets/placeholder_image.webp";
import { BACKEND_URL } from '$lib/utils-server';

export let load: PageServerLoad = async ({ fetch, params }) => {
    let bookResponse = await fetch(BACKEND_URL + "/book/" + params.isbn);
    let coverImage = BACKEND_URL + "/book_cover/" + params.isbn + ".webp";
    if (bookResponse.ok) {
        coverImage = await fetch(coverImage, { method: "HEAD" })
            .then(res => res.ok ? coverImage : placeHolderImage)
            .catch(_ => placeHolderImage)
    } else {
        console.log(await bookResponse.text())
        throw error(bookResponse.status)
    }
    let json = await bookResponse.json();

    let shelvesString = await fetch(BACKEND_URL + "/get_shelves")
        .then(resp => resp.text())
        .catch(_ => "");

    return {
        book: json.book,
        copies: json.copies,
        cover: coverImage,
        shelves: shelvesString === "" ? [] : shelvesString.split(",")
    }
};
