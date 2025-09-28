<script lang="ts">
    import type { QueueQueue } from '$lib/apis/apelle';
    import { dayjs, durationjs } from '$lib/time';
    import QueuedSongCard from './QueuedSongCard.svelte';
    import { _ } from 'svelte-i18n';

    let {
        songs
    }: {
        songs: QueueQueue;
    } = $props();

    const sortedSongs = $derived(
        Object.entries(songs).sort(([_0, a], [_1, b]) => {
            if (a.likes != b.likes) {
                return a.likes - b.likes;
            }
            return dayjs(b.queued_at).diff(dayjs(a.queued_at));
        })
    );
</script>

{#if sortedSongs.length > 0}
    <ol class="flex list-none flex-col gap-3">
        {#each sortedSongs as [id, song] (id)}
            <li class="h-[99px] w-full">
                <QueuedSongCard {song} />
            </li>
        {/each}
    </ol>
{:else}
    <span>{$_('backoffice.queue.empty')}</span>
{/if}
