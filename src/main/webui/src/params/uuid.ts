import type { Uuid } from '$lib/apis/apelle';
import type { ParamMatcher } from '@sveltejs/kit';

export const match = ((param: string): param is Uuid => {
	return /^[0-9A-Fa-f]{8}(-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}$/.test(param);
}) satisfies ParamMatcher;
