import { backendPOST } from '$lib/utils-server';
import { json, type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ cookies, request }) => {
    const { copy_id, start_date, end_date } = await request.json();

    let reservationData = {
        copy_id: copy_id,
        start: start_date,
        end: end_date,
    } 
    await backendPOST(cookies, "/reserve_physical_book", reservationData)

    return json({ success: true });
};
