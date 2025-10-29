<script lang="ts">
    import '../app.css';

    import { isLoading } from 'svelte-i18n';
    import type { Snippet } from 'svelte';
    import Snackbar from '$lib/components/Snackbar.svelte';
    import { global } from '$lib/errors.svelte';
    import authService from '$lib/auth.svelte';
    import Auth from './auth/Auth.svelte';

    let { children }: { children: Snippet } = $props();
</script>

{#if !$isLoading}
    {#if authService.authenticated()}
        {@render children()}
    {:else}
        <Auth />
    {/if}
{/if}

<Snackbar bind:this={global.snackbar} />
