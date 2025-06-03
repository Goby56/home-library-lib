import type { PageLoad } from './$types';

import { error } from "@sveltejs/kit";

import placeHolderImage from "$lib/assets/placeholder_image.webp";

export const load: PageLoad = async ({ params }) => {
    let response = await fetch("http://192.168.1.223:8080/book/" + params.isbn)
    let coverImage = "http://192.168.1.223:8080/book-cover/" + params.isbn + ".webp";
    if (response.ok) {
        coverImage = await fetch(coverImage, { method: "HEAD" })
            .then(res => res.ok ? coverImage : placeHolderImage)
            .catch(_ => placeHolderImage)
    } else {
        throw error(response.status)
    }
    let json = await response.json();
    return {
        book: json.book,
        copies: json.copies,
        cover: coverImage
    }
}
