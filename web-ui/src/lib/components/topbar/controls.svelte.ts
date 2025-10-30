import type { Snippet } from 'svelte';
import { v4 as makeId } from 'uuid';

export interface TopbarControlProps {
    location: 'nav' | 'menu';
    order: number;
    children: Snippet;
}

export const controls: Record<string, TopbarControlProps> = $state({});

/**
 * Register a control in the topbar
 *
 * @param control The control to registe
 * @returns Cleanup function to remove that control
 */
export function register(control: TopbarControlProps): () => void {
    const id = makeId();
    controls[id] = control;
    return () => delete controls[id];
}
