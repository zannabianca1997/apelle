<script lang="ts">
    import { goto } from '$app/navigation';
    import authService, { routeToAuth } from '$lib/auth.svelte';
    import Topbar from '$lib/components/topbar/Topbar.svelte';
    import TopbarControl from '$lib/components/topbar/TopbarControl.svelte';
    import type { Snippet } from 'svelte';
    import { _ } from 'svelte-i18n';

    let { children }: { children: Snippet } = $props();

    async function logOut() {
        authService.signout();
        goto('/auth');
    }
</script>

<TopbarControl location="menu" order={-1}>
    <button onclick={logOut}>{$_('navbar.logout')}</button>
</TopbarControl>

<Topbar />
{@render children()}
