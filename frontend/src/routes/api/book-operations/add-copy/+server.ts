import { backendPOST } from '$lib/utils-server';
import { json, type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ cookies, request }) => {
    const { uuid, shelf_name } = await request.json();

    let physical_copy = {
        uuid, name: shelf_name
    }

    await backendPOST(cookies, "/add_physical_book", physical_copy);

    return json({ success: true });
};
