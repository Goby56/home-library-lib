import { BACKEND_URL } from '$lib/utils-server';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch, url }) => {
    const search = url.searchParams.get("search");
    const searchQueryParam = search ? "&search_str=" + search : "";
    let response = await fetch(BACKEND_URL + "/books?only_physical=false" + searchQueryParam);
    let books: any[] = [];
    if (response.status == 200) {
       books = await response.json();
    }
    return { books }
};
