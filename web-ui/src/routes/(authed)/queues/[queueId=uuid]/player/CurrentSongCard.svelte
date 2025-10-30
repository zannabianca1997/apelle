<script lang="ts">
    import {
        queuesNext,
        queuesPause,
        queuesPlay,
        QueueUserAction,
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

    const canAutoNext = $state(permissions.includes('AUTO_NEXT_SONG'));

    const stopped = $derived('position' in song);
    const duration = $derived(
        dayjs.duration(dayjs.duration(song.song.duration).asMilliseconds())
    );
    const position = $derived(
        'position' in song
            ? dayjs.duration(dayjs.duration(song.position).asMilliseconds())
            : dayjs.duration($time.diff(dayjs(song.starts_at)))
    );
    const ended = $derived(!stopped && position >= duration);

    let autoNext = $state(canAutoNext);

    // Restart the song when ended
    $effect(() => {
        // Can we autonext? and has the song ended?
        if (!autoNext || !ended) {
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
</script>

{#if canAutoNext}
    <TopbarControl location="menu" order={0}>
        <TopBarToggle bind:value={autoNext}>
            {$_('navbar.autoplay')}
        </TopBarToggle>
    </TopbarControl>
{/if}

<hgroup>
    <h1>{song.song.title}</h1>
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
