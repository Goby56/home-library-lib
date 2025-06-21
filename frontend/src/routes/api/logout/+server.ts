import { BACKEND_URL } from '$lib/utils-server';
import { redirect, type RequestHandler } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ fetch, cookies }) => {
    const response = await fetch(BACKEND_URL + "/logout_user", { method: "POST" });
    
    console.log(await response.text())
    
    cookies.delete("session-token", { path: "/" });

    throw redirect(302, "/login");
};
