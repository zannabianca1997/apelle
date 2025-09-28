import type { ParamMatcher } from '@sveltejs/kit';

export const match: ParamMatcher = (s) =>
  /^[0-9A-Fa-f]{8}(-[0-9A-Fa-f]{4}){3}-[0-9A-Fa-f]{12}$/.test(s);
