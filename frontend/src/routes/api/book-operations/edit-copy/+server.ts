import { backendPOST } from '$lib/utils';
import { json, type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ cookies, request }) => {
    const { copy_id, new_shelf_name } = await request.json();

    let edit_data = {
        copy_id, new_shelf_name
    }

    let response = await backendPOST(cookies, "/edit_physical_book", edit_data);
    console.log(response.data);

    return json({ success: true });
};
