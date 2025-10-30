<script lang="ts">
    import { _ } from 'svelte-i18n';
    import { controls } from './controls.svelte';
    import TopBarButton from './controls/TopBarButton.svelte';

    let hovered: boolean = $state(false);
    let opened: boolean = $state(false);

    let menuControls = $derived(
        Object.entries(controls)
            .filter(([_, c]) => c.location == 'menu')
            .toSorted(([_0, a], [_1, b]) => b.order - a.order)
    );
</script>

{#if menuControls.length > 0}
    <div
        class="relative flex h-full items-center justify-center"
        onmouseenter={() => (hovered = true)}
        onmouseleave={() => (hovered = false)}
        role="menu"
        tabindex="-1"
    >
        <TopBarButton onclick={() => (opened = !opened)}>
            {$_('navbar.dropdown')}
        </TopBarButton>
        {#if hovered || opened}
            <menu
                class="left absolute right-0 top-full flex flex-col gap-1 whitespace-nowrap border border-gray-50 bg-gray-600"
            >
                {#each menuControls as [id, { children }] (id)}
                    <li>{@render children()}</li>
                {/each}
            </menu>
        {/if}
    </div>
{/if}
