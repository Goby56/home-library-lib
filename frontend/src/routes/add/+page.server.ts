import type { PageServerLoad, Actions } from "./$types.js";
import { fail } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { zod } from "sveltekit-superforms/adapters";
import { bookFormSchema } from "./book-form-schema";

export const load: PageServerLoad = async () => {
  return {
    form: await superValidate(zod(bookFormSchema)),
  };
};

export const actions: Actions = {
  default: async (event) => {
    const form = await superValidate(event, zod(bookFormSchema));
    if (!form.valid) {
      return fail(400, {
        form,
      });
    }
    console.log(form)
    // Send to backend
    return {
      form,
    };
  },
};
