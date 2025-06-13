import { z } from "zod";
 
export const userCredentialsSchema = z.object({
  username: z.string().min(2).max(30),
  password: z.string().min(4).max(100)
});
export type FormSchema = typeof userCredentialsSchema;
