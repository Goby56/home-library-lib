import { z } from "zod";

const imageFileValidator = (file: File | undefined) => {
    if (file) {
        const acceptedTypes = ['image/webp', 'image/jpeg', 'image/png'];
        return acceptedTypes.includes(file.type) 
    }
    return true;
};

export const bookFormSchema = z.object({
    isbn: z.string().min(10).max(13),
    title: z.string().min(1),
    authors: z.string().min(1),
    genres: z.string(),
    publication_year: z.number().default(new Date().getFullYear()),
    page_count: z.number(),
    language: z.string().min(2),
    cover: z.instanceof(File).optional().refine(imageFileValidator, {
      message: "The file must be an image of format WebP, JPEG, or PNG"
    })
})

export type FormSchema = typeof bookFormSchema;
