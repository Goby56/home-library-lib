import { BACKEND_URL } from '$lib/utils-server';
import { type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ fetch, url }) => {
    const newColor = url.searchParams.get("new_color");
    // Hex color is without #
    return await fetch(BACKEND_URL + "/change_personal_color?new=" + newColor, { method: "POST" });
};
