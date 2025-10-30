<script lang="ts">
    import {
        queuesNext,
        type Current,
        type IdOrRepSongOneOf,
        type QueueCurrent,
        type TimeRef
    } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    import CurrentSongCard, {
        type PlayerProps
    } from './CurrentSongCard.svelte';
    import isString from '$lib/utils/isString';
    import ActionTab, { type Action } from '$lib/components/ActionTab.svelte';
    import IconNext from '~icons/mdi/skip-next';

    const {
        song,
        permissions,
        ...playerProps
    }: Omit<PlayerProps, 'song' | 'actionTabProps' | 'nextAction'> & {
        song?: QueueCurrent;
    } = $props();

    function isLoaded(current: Current): current is TimeRef & {
        song: IdOrRepSongOneOf;
    } {
        return !isString(current.song);
    }

    const actionTabProps: PlayerProps['actionTabProps'] = {
        permissions,
        iconsSize: 48,
        direction: 'col'
    };

    const nextAction = {
        permission: 'NEXT_SONG',
        label: $_('backoffice.currentSong.actions.next'),
        Icon: IconNext,
        onclick() {
            queuesNext(
                playerProps.queueId,
                { auto: false },
                {
                    headers: {
                        'If-Match': `"${playerProps.playerStateId}"`
                    }
                }
            );
        }
    } satisfies Action;
</script>

{#if !song}
    <span>
        {$_('backoffice.currentSong.nothingPlaying')}
    </span>
    <ActionTab
        actions={[
            {
                ...nextAction,
                label: $_('backoffice.currentSong.actions.play_first')
            }
        ]}
        {...actionTabProps}
    />
{:else if !isLoaded(song)}
    <span>
        {$_('backoffice.currentSong.loading')}
    </span>
{:else}
    <CurrentSongCard
        {song}
        {permissions}
        {...playerProps}
        {actionTabProps}
        {nextAction}
    />
{/if}
