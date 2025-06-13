import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import placeHolderImage from "$lib/assets/placeholder_image.webp";
import axios from "axios";
import type { CalendarDate } from "@internationalized/date";
import type { Cookies } from "@sveltejs/kit";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function getLabelFromLanguageCode(code: string): string | undefined {
    return languageCodes.find(lang => lang.value === code)?.label;
}

export const languageCodes = [
    { label: "Svenska", value: "sv" },
    { label: "Engelska", value: "en" },
    { label: "Franska", value: "fr" },
    { label: "Tyska", value: "de" },
    { label: "Spanska", value: "es" },
    { label: "Portugisiska", value: "pt" },
    { label: "Ryska", value: "ru" },
    { label: "Japanska", value: "ja" },
    { label: "Koreanska", value: "ko" },
    { label: "Kinesiska", value: "zh" }
];

export const BACKEND_URL = "http://192.168.1.223:8080";

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

export async function getCoverImage(isbn: string) {
    let coverImage = BACKEND_URL + "/book_cover/" + isbn + ".webp";
    coverImage = await fetch(coverImage, { method: "HEAD" })
        .then(res => res.ok ? coverImage : placeHolderImage)
        .catch(_ => placeHolderImage)
    return coverImage;
}
