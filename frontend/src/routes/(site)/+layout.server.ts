import { error, redirect } from "@sveltejs/kit";
import type { LayoutServerLoad } from "./$types";
import { BACKEND_URL } from "$lib/utils-server";

export const load: LayoutServerLoad = async ({ fetch }) => {
    let response = await fetch(BACKEND_URL + "/get_user");
    
    if (response.status == 401) {
        console.log(response)
        console.log(await response.text())
        redirect(302, "/login");
    }
    if (!response.ok) {
        throw error(response.status)
    }
    return {
        user: await response.json()
    }
}
