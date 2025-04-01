import type { LayoutLoad } from './$types';
import { goto } from '$app/navigation';
import authService from '$lib/auth.svelte';

export const load: LayoutLoad = async ({ url }) => {
	if (authService.authenticated()) {
		const originalUrl = url.searchParams.get('original') ?? '/';
		goto(originalUrl);
	}
};
