import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch, url }) => {
    const isbn = url.searchParams.get("isbn");
     
    let response = await fetch(BACKEND_URL + "/book_cover/" + isbn + ".webp");
    console.log(response);
    return response;
};
