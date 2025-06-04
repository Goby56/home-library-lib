import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import placeHolderImage from "$lib/assets/placeholder_image.webp";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function getLabelFromLanguageCode(code: string): string | undefined {
    return languageCodes.find(lang => lang.value === code)?.label;
}

export const languageCodes = [
    { label: "Svenska", value: "se" },
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


export async function getCoverImage(isbn: string) {
    let coverImage = "http://192.168.1.223:8080/book-cover/" + isbn + ".webp";
    coverImage = await fetch(coverImage, { method: "HEAD" })
        .then(res => res.ok ? coverImage : placeHolderImage)
        .catch(_ => placeHolderImage)
    return coverImage;
}

