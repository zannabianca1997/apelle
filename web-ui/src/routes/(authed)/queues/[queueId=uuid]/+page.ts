import type { PageLoad } from './$types';
import authService, { routeToAuth } from '$lib/auth.svelte';

export const load: PageLoad = async ({ url }) => {
    if (!authService.authenticated()) {
        await routeToAuth(url);
    }
};
