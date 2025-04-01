import { _ } from 'svelte-i18n'

/**
 * Signal an error to the user.
 * 
 * @param msg The error message
 */
export async function error(msg: string): Promise<void> {
    console.error(msg);
}
