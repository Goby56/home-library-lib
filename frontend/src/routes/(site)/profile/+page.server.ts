import type { PageServerLoad } from './$types';

import { error } from "@sveltejs/kit";

import { BACKEND_URL } from '$lib/utils-server';

export let load: PageServerLoad = async ({ fetch }) => {
    let reservations = await fetch(BACKEND_URL + "/get_user_reservations");
    if (reservations.ok) {
        return {
            user_reservations: await reservations.json()
        }
    } else {
        console.log(await reservations.text())
        throw error(reservations.status)
    }
};

