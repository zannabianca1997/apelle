<script lang="ts">
    import { songsGet, type QueuedSong } from '$lib/apis/apelle';
    import isString from '$lib/utils/isString';
    import { _ } from 'svelte-i18n';

    let {
        song
    }: {
        song: QueuedSong;
    } = $props();

    $effect(() => {
        if (!isString(song.song)) {
            return;
        }

        songsGet(song.song, {
            source_data: true
        }).then(({ data }) => (song.song = data));
    });
</script>

<div>
    {#if !isString(song.song)}
        <span>{song.song.title}</span>
    {:else}
        <span>{$_('backoffice.song.loading')}</span>
    {/if}
</div>
