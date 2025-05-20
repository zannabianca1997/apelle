import type { Snippet } from 'svelte';

export interface NavBarElement {
	readonly order: number;
	readonly content: Snippet;
}

export const navElements: Record<string, NavBarElement> = $state({});
export const navMenuElements: Record<string, NavBarElement> = $state({});
