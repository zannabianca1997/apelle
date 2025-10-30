<script lang="ts">
    import '../app.css';

    import { isLoading } from 'svelte-i18n';
    import type { Snippet } from 'svelte';
    import Snackbar from '$lib/components/Snackbar.svelte';
    import { global } from '$lib/errors.svelte';
    import authService from '$lib/auth.svelte';
    import Auth from './auth/Auth.svelte';
    import type { Snapshot } from './$types';

    let { children }: { children: Snippet } = $props();

    let auth: Auth | undefined = $state();

    export const snapshot: Snapshot<{
        auth?: Auth['snapshot'] extends Snapshot<infer T> ? T : never;
    }> = {
        capture: () => ({
            auth: auth?.snapshot.capture()
        }),

        restore: ({ auth: authData }) => {
            if (authData) {
                auth?.snapshot.restore(authData);
            }
        }
    };
</script>

{#if !$isLoading}
    {#if authService.authenticated()}
        {@render children()}
    {:else}
        <Auth bind:this={auth} />
    {/if}
{/if}

<Snackbar bind:this={global.snackbar} />
