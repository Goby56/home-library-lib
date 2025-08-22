import { z } from "zod";

const imageFileValidator = (file: File | undefined) => {
    if (file) {
        const acceptedTypes = ['image/webp', 'image/jpeg', 'image/png'];
        return acceptedTypes.includes(file.type) 
    }
    return true;
};

export const bookFormSchema = z.object({
    isbn: z.string().nullable(),
    title: z.string().min(1),
    authors: z.string().min(1),
    genres: z.string().nullable(),
    publication_year: z.number().nullable(),
    page_count: z.number().nullable(),
    language: z.string().min(2).nullable(),
    cover: z.instanceof(File).optional().refine(imageFileValidator, {
      message: "The file must be an image of format WebP, JPEG, or PNG"
    })
})

export type FormSchema = typeof bookFormSchema;
