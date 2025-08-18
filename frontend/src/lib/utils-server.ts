import axios from "axios";
import type { Cookies } from "@sveltejs/kit";
import { env } from "$env/dynamic/private";
export const BACKEND_URL = env.BACKEND_URL;

export async function backendPOST(cookies: Cookies, endpoint: string, payload: any) {
    const sessionToken = cookies.get("session-token");
    return await axios.post(BACKEND_URL + endpoint, payload, {
        headers: {
            Cookie: `session-token=${sessionToken}`
        }
    })
}

export function setSessionCookie(cookies: Cookies, session: any) {
    cookies.set("session-token", session, {
        path: "/",
        httpOnly: true,
        secure: false, // TODO Change to true when switched to HTTPS
        sameSite: "lax",
        maxAge: 60 * 60 * 24 * 7
    })
}
