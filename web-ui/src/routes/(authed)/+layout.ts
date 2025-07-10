import type { LayoutLoad } from './$types';
import authService, { routeToAuth } from '$lib/auth.svelte';
import { Logger } from '$lib/logger';

const logger = new Logger('routes.authed.layout');

/**
 * Redirect to the auth endpoint if the user is not authenticated
 */
export const load: LayoutLoad = async ({ url }) => {
	if (!authService.authenticated()) {
		await routeToAuth(url);
	}
};
