import type { LayoutLoad } from './$types';
import authService from '$lib/auth.svelte';
import { goto } from '$app/navigation';

/**
 * Redirect to the auth endpoint if the user is not authenticated
 */
export const load: LayoutLoad = async ({ url }) => {
	if (!authService.authenticated()) {
		const authUrl = new URL('/auth', url);
		authUrl.searchParams.set('original', url.toString());
		goto(authUrl);
	}
};
