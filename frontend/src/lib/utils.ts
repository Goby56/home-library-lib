import { parseAbsoluteToLocal } from "@internationalized/date";
import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export function getLabelFromLanguageCode(code: string): string | undefined {
    return languageCodes.find(lang => lang.value === code)?.label;
}

export function reservationDuration(reservation: any): string {
    return `${getDateString(reservation.start_date)} - ${getDateString(reservation.end_date)}`;
}

function getDateString(dbDateString: string) {
  return parseAbsoluteToLocal(dbDateString).toDate().toLocaleDateString();
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


