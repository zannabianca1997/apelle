import { isUuid } from '$lib/matchers';
import type { ParamMatcher } from '@sveltejs/kit';

export const match = isUuid satisfies ParamMatcher;
