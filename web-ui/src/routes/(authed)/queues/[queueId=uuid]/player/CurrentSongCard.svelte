<script lang="ts">
    import type { IdOrRepSongOneOf, TimeRef } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    import { dayjs, time } from '$lib/time';
    import TopbarControl from '$lib/components/topbar/TopbarControl.svelte';
    import TopBarToggle from '$lib/components/topbar/controls/TopBarToggle.svelte';

    let {
        song,
        canAutoNext
    }: {
        song: TimeRef & { song: IdOrRepSongOneOf };
        canAutoNext: boolean;
    } = $props();

    const stopped = $derived('position' in song);
    const duration = $derived(
        dayjs.duration(dayjs.duration(song.song.duration).asMilliseconds())
    );
    const position = $derived(
        'position' in song
            ? dayjs.duration(dayjs.duration(song.position).asMilliseconds())
            : dayjs.duration($time.diff(dayjs(song.starts_at)))
    );

    let autoNext = $state(canAutoNext);

    // Restart the song when ended
    $effect(() => {
        // Check if the song ended
        const ended = !stopped && position > duration;

        if (!ended) {
            return;
        }
    });
</script>

{#if canAutoNext}
    <TopbarControl location="menu" order={0}>
        <TopBarToggle bind:value={autoNext}>
            {$_('navbar.autoplay')}
        </TopBarToggle>
    </TopbarControl>
{/if}

<div>
    <h1>{song.song.title}</h1>
    <span>
        {$_('backoffice.currentSong.progress', {
            values: {
                position: position.format($_('backoffice.song.durationFormat')),
                duration: duration.format($_('backoffice.song.durationFormat'))
            }
        })}
    </span>
</div>
