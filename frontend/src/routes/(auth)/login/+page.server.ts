import type { PageServerLoad, Actions } from "./$types.js";
import { fail, error, redirect } from "@sveltejs/kit";
import { superValidate } from "sveltekit-superforms";
import { userCredentialsSchema } from "./../user-credentials-schema.js";
import { zod } from "sveltekit-superforms/adapters";
import axios from "axios";
import { BACKEND_URL } from "$lib/utils.js";
 
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

    let loginCredentials = {
        username: form.data.username,
        password: form.data.password,
    }

    let response = await axios.post(BACKEND_URL + "/login_user", loginCredentials)
        .catch(resp => {
            return resp;
        });

    if (response.status == 401) {
       form.errors.username = ["Användarnamn eller lösenord stämmer inte"];
       return fail(401, {form})
    }

    if (response.status == 200) {
        event.cookies.set("session-token", response.data, {
            path: "/",
            httpOnly: true,
            secure: false, // TODO Change to true when switched to HTTPS
            sameSite: "lax",
            maxAge: 60 * 60 * 24 * 7
        })
        redirect(303, "/");
    }

    return {
      form
    };
  },
};
