<script lang="ts">
    import {
        configsGet,
        type Queue,
        type QueueUserAction,
        type SearchResponseItem
    } from '$lib/apis/apelle';
    import Search from './search/SearchSection.svelte';
    import type { PageProps } from './$types';
    import Player from './player/Player.svelte';
    import QueueView from './queue/QueueView.svelte';
    import isString from '$lib/utils/isString';
    import { _ } from 'svelte-i18n';
    import Connection from '$lib/queue.svelte';
    import { goto } from '$app/navigation';
    import { error } from '$lib/errors.svelte';

    const { data }: PageProps = $props();
    const queueId = $derived(data.queueId);
    const connection = $derived(new Connection(queueId, notFound));

    let queue: Queue | null = $derived(connection.queue);

    // Fetch the queue config if it was not provided
    $effect(() => {
        if (!queue || !isString(queue.config)) {
            return;
        }

        configsGet(queue.config).then(({ data }) => (queue.config = data));
    });

    const permissions: QueueUserAction[] = $derived(
        queue && !isString(queue.config)
            ? queue.config.roles[queue.user.role].permissions
            : []
    );

    function notFound() {
        error({
            _tag: 'queueNotFound',
            msg: $_('backoffice.notFound', {
                values: {
                    id: queueId
                }
            })
        });
        goto('/');
    }

    const titleClasses =
        'text-[32px] font-black leading-[1.5] tracking-[.01em] text-[#379b46]';
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
            <Player
                song={queue.current}
                canAutoNext={permissions.includes('AUTO_NEXT_SONG')}
            />
        </section>
        <section>
            <h1 class={titleClasses}>
                {$_('backoffice.partyName')}
                <code class="text-white">{queue.code}</code>
            </h1>
            {#if permissions.includes('ENQUEUE_SONG')}
                <Search queueId={queue.id} />
            {/if}
        </section>
        <section>
            <h1 class={titleClasses}>
                {$_('backoffice.queue.title')}
            </h1>
            <QueueView
                bind:songs={queue.queue}
                queueId={queue.id}
                {permissions}
            />
        </section>
    </main>

    <!-- <aside class="w-full">
        <pre>{JSON.stringify(queue, undefined, 2)}</pre>
    </aside> -->
{:else}
    <h1>{$_('backoffice.loading')}</h1>
{/if}
