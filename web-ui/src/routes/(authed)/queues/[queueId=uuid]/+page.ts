import type { PageLoad } from './$types';
import authService, { routeToAuth } from '$lib/auth.svelte';
import Connection from '$lib/queue.svelte';

export const load: PageLoad = async ({ params: { queueId }, url }) => {
	if (!authService.authenticated()) {
		await routeToAuth(url);
	}

	const connection = new Connection(queueId);

	return { connection };
};
