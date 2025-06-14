import { BACKEND_URL } from '$lib/utils';
import { type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch, url }) => {
    const isbn = url.searchParams.get("isbn");

    return await fetch(BACKEND_URL + "/book_cover/" + isbn + ".webp");
};
