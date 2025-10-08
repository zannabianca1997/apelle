<script lang="ts">
    import type { Queue, SearchResponseItem } from '$lib/apis/apelle';
    import Search from './search/SearchSection.svelte';
    import type { PageProps } from './$types';
    import Player from './player/Player.svelte';
    import QueueView from './queue/QueueView.svelte';
    import isString from '$lib/utils/isString';
    import { _ } from 'svelte-i18n';

    const { data }: PageProps = $props();

    const queueId = $derived(data.connection.queue?.id);
    let queue: Queue | null = $derived(data.connection.queue);
</script>

<svelte:head>
    {#if queue}
        {#if queue.current && !isString(queue.current.song)}
            <title>Apelle - {queue.current.song.title}</title>
        {:else}
            <title>Apelle - {queue.code}</title>
        {/if}
    {/if}
</svelte:head>

{#if queue}
    <main class="flex w-4/5 flex-col gap-[57px]">
        <section
            class="flex h-[244px] w-full flex-row items-center justify-evenly gap-6 rounded-md bg-gradient-to-r from-[rgba(55,155,70,0.75)] to-[rgba(36,101,46,0.75)] p-3"
        >
            <Player song={queue.current} />
        </section>
        <section>
            <h1
                class="text-[32px] font-black leading-[1.5] tracking-[.01em] text-[#379b46]"
            >
                {$_('backoffice.partyName')}
                <code class="text-white">{queue.code}</code>
            </h1>
            <Search queueId={queue.id} />
        </section>
        <section>
            <h1
                class="text-[32px] font-black leading-[1.5] tracking-[.01em] text-[#379b46]"
            >
                {$_('backoffice.queue.title')}
            </h1>
            <QueueView songs={queue.queue} />
        </section>
    </main>

    <aside>
        {JSON.stringify(queue, undefined, 2)}
    </aside>
{:else}
    <h1>{$_('backoffice.loading')}</h1>
{/if}
