import { z } from "zod";

export const bookFormSchema = z.object({
    isbn: z.string().min(10).max(13),
    title: z.string().min(1),
    authors: z.array(z.string().min(1)).min(1).default([""]),
    publication_year: z.number().default(new Date().getFullYear()),
    language: z.string().min(2),
    page_count: z.number(),
    genres: z.array(z.string().min(1)).default([""]),
})

export type FormSchema = typeof bookFormSchema;
