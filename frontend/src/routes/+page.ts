import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ fetch, params }) => {
    return {
        books: await fetch("http://192.168.1.223:8080/books?include_non_physical=true").then((data) => data.json())
    }
}
