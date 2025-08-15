import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ fetch, url }) => {
    const id = url.searchParams.get("id");

    return await fetch(BACKEND_URL + "/remove_reservation/" + id, { method: "POST" });
};
