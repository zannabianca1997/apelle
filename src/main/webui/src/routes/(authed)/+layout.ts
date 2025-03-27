import type { LayoutLoad } from './$types'
import { authenticated } from '$lib/auth.svelte'
import { goto } from '$app/navigation'

export const load: LayoutLoad = async ({ url }) => {
    if (!authenticated()) {
        const authUrl = new URL('/authenticate', url);
        authUrl.searchParams.set('original', url.toString());
        goto(authUrl)
    }
}