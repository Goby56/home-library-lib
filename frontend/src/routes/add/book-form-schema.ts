import { z } from "zod";

export const bookFormSchema = z.object({
    title: z.string().min(1),
    authors: z.array(z.string().min(1)).min(1).default([""]),
    published_date: z.string(),
    language: z.string().min(2),
    page_count: z.number(),
    genres: z.array(z.string().min(1)).default([""]),
})

export type FormSchema = typeof bookFormSchema;
