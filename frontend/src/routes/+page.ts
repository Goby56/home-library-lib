import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    return {
        books: await fetch("http://0.0.0.0:8080/books").then((data) => data.json())
    }
}
