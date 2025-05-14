import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    return {
        books: await fetch("http://192.168.1.223:8080/books").then((data) => data.json())
    }
}
