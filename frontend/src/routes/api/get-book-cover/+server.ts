import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch, url }) => {
    const uuid = url.searchParams.get("uuid");
     
    return await fetch(BACKEND_URL + "/book_cover/" + uuid + ".webp");
};
