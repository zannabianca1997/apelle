<script lang="ts">
    import ApelleIcon from '~icons/mdi/music-clef-treble';

    import Menu from './Menu.svelte';
    import { controls } from './controls.svelte';

    let navControls = $derived(
        Object.entries(controls)
            .filter(([, c]) => c.location == 'nav')
            .toSorted(([, a], [, b]) => b.order - a.order)
    );
</script>

<header
    class="from-backgroundtop to-backgroundbottom absolute top-0 flex w-full flex-row items-center justify-between border-b border-gray-300 bg-gradient-to-b p-3"
>
    <hgroup class="flex flex-row items-center">
        <ApelleIcon height="40" width="40" />
        <h1 class="font-black">Apelle</h1>
    </hgroup>
    <nav class="flex flex-row self-stretch">
        {#each navControls as [id, { children }] (id)}
            <div>{@render children()}</div>
        {/each}
        <Menu />
    </nav>
</header>
