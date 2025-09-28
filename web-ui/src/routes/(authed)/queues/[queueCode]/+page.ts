import { queuesFind } from '$lib/apis/apelle';
import { AxiosError } from 'axios';
import type { PageLoad } from './$types';
import { error } from '@sveltejs/kit';
import { goto } from '$app/navigation';
import normalizeCode from '$lib/utils/normalizeCode';
import authService, { routeToAuth } from '$lib/auth.svelte';

/**
 * This route does not contain an actual page, it is just search the queue by code and redirect to the queue page by id.
 *
 * This is by design so urls copied from the web browser will continue to refer to the same queue, even if the queue code changes.
 */
export const load: PageLoad = async ({ params: { queueCode }, url }) => {
    if (!authService.authenticated()) {
        await routeToAuth(url);
    }
    const code = normalizeCode(queueCode);
    if (!code) {
        await goto('/');
        return;
    }
    let response;
    try {
        response = await queuesFind({ code });
    } catch (e: unknown) {
        if (e instanceof AxiosError) {
            if (e.status == 404) {
                error(404, {
                    message: `Party ${code} not found.`
                });
            }
        }
        throw e;
    }

    goto(`/queues/${response.data}`);
};
