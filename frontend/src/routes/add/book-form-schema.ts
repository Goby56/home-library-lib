import { z } from "zod";

const imageFileValidator = (file: File) => {
    const acceptedTypes = ['image/webp', 'image/jpeg', 'image/png'];
    return acceptedTypes.includes(file.type) 
};

export const bookFormSchema = z.object({
    isbn: z.string().min(10).max(13),
    title: z.string().min(1),
    authors: z.array(z.string().min(1)).min(1).default([""]),
    publication_year: z.number().default(new Date().getFullYear()),
    language: z.string().min(2),
    page_count: z.number(),
    genres: z.array(z.string().min(1)).default([""]),
    cover: z.instanceof(File).refine(imageFileValidator, {
      message: "The file must be an image of format WebP, JPEG, or PNG"
    }),
})

export type FormSchema = typeof bookFormSchema;
