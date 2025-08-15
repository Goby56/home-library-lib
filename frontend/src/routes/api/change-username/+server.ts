import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ fetch, url }) => {
    const newUsername = url.searchParams.get("new_username");
     
    return await fetch(BACKEND_URL + "/change_username?new_username=" + newUsername, { method: "POST" });
};

