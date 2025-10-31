<script lang="ts">
    import {
        queuesNext,
        queuesPause,
        queuesPlay,
        QueueUserAction,
        songsGet,
        type IdOrRepSongOneOf,
        type TimeRef
    } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    import { dayjs, time } from '$lib/time';
    import TopbarControl from '$lib/components/topbar/TopbarControl.svelte';
    import TopBarToggle from '$lib/components/topbar/controls/TopBarToggle.svelte';

    import IconPlay from '~icons/mdi/play';
    import IconPause from '~icons/mdi/pause';

    import type { ComponentProps } from 'svelte';
    import ActionTab, { type Action } from '$lib/components/ActionTab.svelte';
    import sources from '$lib/sources';
    import MarqueeOnHover from '$lib/components/MarqueeOnHover.svelte';
    import { preferences } from '$lib/preferences.svelte';
    import TopBarSlider from '$lib/components/topbar/controls/TopBarSlider.svelte';

    export interface PlayerProps {
        queueId: string;
        permissions: QueueUserAction[];
        playerStateId: string;
        song: TimeRef & { song: IdOrRepSongOneOf };
        actionTabProps: Omit<ComponentProps<typeof ActionTab>, 'actions'>;
        nextAction: Action;
    }

    const {
        queueId,
        song,
        permissions,
        playerStateId,
        actionTabProps,
        nextAction
    }: PlayerProps = $props();

    const canAutoNext = permissions.includes('AUTO_NEXT_SONG');

    const stopped = $derived('position' in song);
    const duration = $derived(
        dayjs.duration(dayjs.duration(song.song.duration).asMilliseconds())
    );
    const position = $derived.by(() => {
        if ('position' in song) {
            return dayjs.duration(
                dayjs.duration(song.position).asMilliseconds()
            );
        }
        const elapsed = dayjs.duration($time.diff(dayjs(song.starts_at)));
        if (elapsed > duration) {
            return duration;
        }
        return elapsed;
    });
    const ended = $derived(!stopped && position >= duration);

    // Restart the song when ended
    $effect(() => {
        // Can we autonext? and has the song ended?
        if (!preferences.autoNext || !ended) {
            return;
        }

        queuesNext(
            queueId,
            { auto: true },
            {
                headers: {
                    'If-Match': `"${playerStateId}"`
                }
            }
        );
    });

    const actions = $derived([
        stopped
            ? {
                  permission: 'PLAY_SONG',
                  label: $_('backoffice.currentSong.actions.play'),
                  Icon: IconPlay,
                  onclick() {
                      queuesPlay(queueId);
                  }
              }
            : {
                  permission: 'PAUSE_SONG',
                  label: $_('backoffice.currentSong.actions.pause'),
                  Icon: IconPause,
                  onclick() {
                      queuesPause(queueId);
                  }
              },
        nextAction
    ] satisfies Action[]);

    const [Thumbnail, tData, Player, songWithDetails] = $derived.by(() => {
        const details = song.song.details;
        if (!details || typeof details === 'string') {
            songsGet(song.song.id, { details: true }).then(({ data }) => {
                song.song = data;
            });

            return [null, null, null, null];
        }

        const songWithDetails = { ...song.song, details };

        return [
            ...sources.songThumbnailData(songWithDetails),
            sources.playerElement(songWithDetails),
            songWithDetails
        ];
    });

    const playerOrThumbSize = {
        width: '100%',
        height: '100%'
    };
</script>

{#if canAutoNext}
    <TopbarControl location="menu" order={0}>
        <TopBarToggle bind:value={preferences.autoNext}>
            {$_('navbar.autoplay')}
        </TopBarToggle>
    </TopbarControl>
{/if}

<TopbarControl location="menu" order={1}>
    <TopBarToggle bind:value={preferences.playFromHere}>
        {$_('navbar.playFromHere')}
    </TopBarToggle>
</TopbarControl>
<TopbarControl location="menu" order={1}>
    <TopBarSlider bind:value={preferences.volume} max={1} min={0} step={0.1}>
        {$_('navbar.volume')}
    </TopBarSlider>
</TopbarControl>

<div
    class="flex min-h-[200px] min-w-[300px] grow-0 items-center justify-center"
>
    {#if preferences.playFromHere && songWithDetails}
        <Player
            song={songWithDetails}
            {...playerOrThumbSize}
            {position}
            {stopped}
            volume={preferences.volume}
        />
    {:else}
        <Thumbnail src={tData} {...playerOrThumbSize} />
    {/if}
</div>

<hgroup class="shrink overflow-auto">
    <MarqueeOnHover host="h1" class="pb-1 text-lg font-semibold">
        {song.song.title}
    </MarqueeOnHover>
    <span>
        {$_('backoffice.currentSong.progress', {
            values: {
                position: position.format($_('backoffice.song.durationFormat')),
                duration: duration.format($_('backoffice.song.durationFormat'))
            }
        })}
    </span>
</hgroup>

<ActionTab {actions} {...actionTabProps} />
