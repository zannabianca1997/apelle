import type { LayoutLoad } from './$types'
import { authenticated } from '$lib/auth.svelte'
import { goto } from '$app/navigation'

export const load: LayoutLoad = async () => {
    if (authenticated()) {
        goto('/')
    }
}