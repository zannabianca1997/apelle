<script lang="ts">
    import type { Current, IdOrRepSongOneOf, TimeRef } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    import { dayjs, durationjs, time } from '$lib/time';
    import { readable } from 'svelte/store';

    let {
        song
    }: {
        song: TimeRef & { song: IdOrRepSongOneOf };
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
</script>

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
