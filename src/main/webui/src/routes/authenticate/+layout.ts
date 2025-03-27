import type { LayoutLoad } from './$types'
import { goto } from '$app/navigation'
import authService from '$lib/auth.svelte'

export const load: LayoutLoad = async () => {
    if (authService.authenticated()) {
        goto('/')
    }
}