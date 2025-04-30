import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params }) => {
    return {
        books: await fetch("http://192.168.1.223:8080/get_books?isbn=" + params.isbn).then((data) => data.json())
    }
}
