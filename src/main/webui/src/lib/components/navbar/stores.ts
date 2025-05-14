import type { Snippet } from 'svelte';
import { writable, type Writable } from 'svelte/store';

export const PageNavBar: Writable<Snippet | undefined> = writable();
