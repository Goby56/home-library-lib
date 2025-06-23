import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const GET: RequestHandler = async ({ fetch, url }) => {
    const searchStr = url.searchParams.get("search");
     
    return await fetch(BACKEND_URL + "/get_search_suggestions?search_str=" + searchStr);
};

