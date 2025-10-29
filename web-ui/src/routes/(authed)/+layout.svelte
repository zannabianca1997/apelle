<script lang="ts">
    import authService from '$lib/auth.svelte';
    import Topbar from '$lib/components/topbar/Topbar.svelte';
    import TopbarControl from '$lib/components/topbar/TopbarControl.svelte';
    import type { Snippet } from 'svelte';
    import { _ } from 'svelte-i18n';

    let { children }: { children: Snippet } = $props();
</script>

<TopbarControl location="menu" order={-1}>
    <button onclick={() => authService.signout()}>{$_('navbar.logout')}</button>
</TopbarControl>

<Topbar />
{@render children()}
