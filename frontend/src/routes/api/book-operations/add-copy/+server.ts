import { backendPOST } from '$lib/utils';
import { json, type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ cookies, request }) => {
    const { isbn, shelf_name } = await request.json();

    let physical_copy = {
        isbn, name: shelf_name
    }

    await backendPOST(cookies, "/add_physical_book", physical_copy);

    return json({ success: true });
};
