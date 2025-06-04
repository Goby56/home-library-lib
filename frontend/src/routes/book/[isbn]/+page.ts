import type { PageLoad } from './$types';

import { error } from "@sveltejs/kit";

import placeHolderImage from "$lib/assets/placeholder_image.webp";

export const load: PageLoad = async ({ fetch, params }) => {
    let bookResponse = await fetch("http://192.168.1.223:8080/book/" + params.isbn)
    let coverImage = "http://192.168.1.223:8080/book-cover/" + params.isbn + ".webp";
    if (bookResponse.ok) {
        coverImage = await fetch(coverImage, { method: "HEAD" })
            .then(res => res.ok ? coverImage : placeHolderImage)
            .catch(_ => placeHolderImage)
    } else {
        throw error(bookResponse.status)
    }
    let json = await bookResponse.json();

    let shelvesString = await fetch("http://192.168.1.223:8080/get-shelves")
        .then(resp => resp.text())
        .catch(_ => "");
    return {
        book: json.book,
        copies: json.copies,
        cover: coverImage,
        shelves: shelvesString === "" ? [] : shelvesString.split(",")
    }
}
