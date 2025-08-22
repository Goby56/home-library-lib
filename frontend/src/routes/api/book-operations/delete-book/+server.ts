import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ fetch, url }) => {
    const uuid = url.searchParams.get("uuid");

    return await fetch(BACKEND_URL + "/delete_book/" + uuid, { method: "POST" });
};
