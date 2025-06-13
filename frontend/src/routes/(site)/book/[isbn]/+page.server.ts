import type { PageServerLoad, Actions } from './$types';

import { error } from "@sveltejs/kit";

import placeHolderImage from "$lib/assets/placeholder_image.webp";
import { BACKEND_URL, backendPOST } from '$lib/utils';
import { parseDate } from '@internationalized/date';

export let load: PageServerLoad = async ({ fetch, params }) => {
    let bookResponse = await fetch(BACKEND_URL + "/book/" + params.isbn)
    let coverImage = BACKEND_URL + "/book_cover/" + params.isbn + ".webp";
    if (bookResponse.ok) {
        coverImage = await fetch(coverImage, { method: "HEAD" })
            .then(res => res.ok ? coverImage : placeHolderImage)
            .catch(_ => placeHolderImage)
    } else {
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


export let actions: Actions = {
  reserve_copy: async ({ cookies, request }) => {
    let data = await request.formData();

    let idRaw = data.get("physicalCopyID");
    let startRaw = data.get("reservationStart");
    let endRaw = data.get("reservationEnd");

    if (!idRaw || !startRaw || !endRaw) {
      return new Response("Missing form data", { status: 400 });
    }

    let copyId = parseInt(idRaw.toString());
    let start = parseDate(startRaw.toString());
    let end = parseDate(endRaw.toString());

    let reservationData = {
        copy_id: copyId,
        start: start.toString(),
        end: end.toString(),
    } 
    await backendPOST(cookies, "/reserve_physical_book", reservationData)

    return { success: true };
  },
  move_copy: async ({ cookies, request }) => {
    let data = await request.formData();

    let copy_id = data.get("physicalCopyID");

    let edit_data = {
        copy_id, new_shelf_name: ""
    }

    await backendPOST(cookies, "/edit_physical_book", edit_data);

    return { success: true };
  },
  remove_copy: async ({ cookies, request }) => {
    let data = await request.formData();

    let copy_id = data.get("physicalCopyID");

    let edit_data = {
        copy_id, new_shelf_name: ""
    }

    await backendPOST(cookies, "/edit_physical_book", edit_data);

    return { success: true };
  },
  add_copy: async ({ cookies, request }) => {
    let data = await request.formData();

    let isbn = data.get("bookIsbn");
    let name = data.get("shelfName");

    let physical_copy = {
        isbn, name
    }

    await backendPOST(cookies, "/add_physical_book", physical_copy);

    return { success: true };
  }
};
