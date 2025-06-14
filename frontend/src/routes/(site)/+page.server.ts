import { BACKEND_URL } from '$lib/utils';
import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, params }) => {

    let response = await fetch(BACKEND_URL + "/books?include_non_physical=true");
    
    if (response.status == 401) {
        console.log(await response.text())
        redirect(302, "/login");
    }
    
    let books: any[] = [];
    if (response.status == 200) {
       books = await response.json();
    }

    return { books }
};
