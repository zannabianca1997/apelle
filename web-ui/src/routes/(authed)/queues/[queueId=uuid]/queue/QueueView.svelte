<script lang="ts">
    import type { QueueQueue, QueueUserAction } from '$lib/apis/apelle';
    import { dayjs } from '$lib/time';
    import QueuedSongCard from './QueuedSongCard.svelte';
    import { _ } from 'svelte-i18n';

    const {
        queueId,
        songs = $bindable(),
        permissions
    }: {
        queueId: string;
        songs: QueueQueue;
        permissions: QueueUserAction[];
    } = $props();

    const sortedSongs = $derived(
        Object.entries(songs).sort(([_0, a], [_1, b]) => {
            if (a.likes != b.likes) {
                return b.likes - a.likes;
            }
            return dayjs(a.queued_at).diff(dayjs(b.queued_at));
        })
    );
</script>

{#if sortedSongs.length > 0}
    <ol class="flex list-none flex-col gap-3">
        {#each sortedSongs as [id, _], i (id)}
            <QueuedSongCard {queueId} bind:song={songs[id]} {permissions} />
        {/each}
    </ol>
{:else}
    <span>{$_('backoffice.queue.empty')}</span>
{/if}
