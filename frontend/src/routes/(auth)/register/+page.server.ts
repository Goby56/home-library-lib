import type { PageServerLoad, Actions } from "./$types.js";
import { error, fail, redirect } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { userCredentialsSchema } from "./../user-credentials-schema.js";
import { zod } from "sveltekit-superforms/adapters";
import axios from "axios";
import { BACKEND_URL, setSessionCookie } from "$lib/utils.js";
 
export const load: PageServerLoad = async () => {
 return {
  form: await superValidate(zod(userCredentialsSchema)),
 };
};

export const actions: Actions = {
  default: async (event: any) => {
    const form = await superValidate(event, zod(userCredentialsSchema));
    if (!form.valid) {
      return fail(400, {
        form,
      });
    }

    let registerCredentials = {
        username: form.data.username,
        password: form.data.password,
    }

    let response = await axios.post(BACKEND_URL + "/register_user", registerCredentials)
        .catch(resp => {
            return resp;
        });

    if (response.status == 409) {
       form.errors.username = ["Användarnamnet är redan taget"];
       return fail(409, {form})
    }

    console.log(response)

    if (response.status == 200) {
        setSessionCookie(event.cookies, response.data);
        redirect(303, "/");
    }

    return {
      form, token: response.data
    };
  },
};
