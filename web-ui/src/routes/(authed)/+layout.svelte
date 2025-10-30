<script lang="ts">
    import authService from '$lib/auth.svelte';
    import Button from '$lib/components/forms/Button.svelte';
    import TopBarButton from '$lib/components/topbar/controls/TopBarButton.svelte';
    import Topbar from '$lib/components/topbar/Topbar.svelte';
    import TopbarControl from '$lib/components/topbar/TopbarControl.svelte';
    import type { Snippet } from 'svelte';
    import { _ } from 'svelte-i18n';

    let { children }: { children: Snippet } = $props();
</script>

<TopbarControl location="menu" order={-1}>
    <TopBarButton onclick={() => authService.signout()}>
        {$_('navbar.logout')}
    </TopBarButton>
</TopbarControl>

<Topbar />
{@render children()}
