import { BACKEND_URL } from '$lib/utils-server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {
    let response = await fetch(BACKEND_URL + "/books?only_physical=false");
    let books: any[] = [];
    if (response.status == 200) {
       books = await response.json();
    }
    return { books }
};
