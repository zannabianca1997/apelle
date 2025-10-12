<script lang="ts">
    import {
        queuesLike,
        queuesNext,
        QueueUserAction,
        songsGet,
        type QueuedSong
    } from '$lib/apis/apelle';
    import { dayjs } from '$lib/time';
    import isString from '$lib/utils/isString';
    import { _ } from 'svelte-i18n';
    import IconRemove from '~icons/mdi/delete-empty-outline';
    import IconBan from '~icons/mdi/cancel';
    import IconPlay from '~icons/mdi/play';
    import IconVotedOnce from '~icons/mdi/chevron-up';
    import IconVotedTwice from '~icons/mdi/chevron-double-up';
    import IconVotedMany from '~icons/mdi/chevron-triple-up';
    import IconMoveUp from '~icons/mdi/arrow-up';
    import MarqueeOnHover from '$lib/components/MarqueeOnHover.svelte';
    import { songThumbnailData } from '$lib/sources';
    import { Logger } from '$lib/logger';
    import type { Component } from 'svelte';
    const logger = new Logger(
        'routes.authed.queues.queueIdUuid.queue.QueuedSongCard'
    );

    const {
        queueId,
        song = $bindable(),
        permissions
    }: {
        queueId: string;
        song: QueuedSong;
        permissions: QueueUserAction[];
    } = $props();

    const songId = $derived(isString(song.song) ? song.song : song.song.id);

    function fetchData() {
        songsGet(songId, {
            details: true
        }).then(({ data }) => (song.song = data));
    }

    $effect(() => {
        if (isString(song.song) || isString(song.song.details)) {
            fetchData();
        }
    });

    const duration = $derived.by(() => {
        if (isString(song.song)) {
            return null;
        }

        return dayjs
            .duration(dayjs.duration(song.song.duration).asMilliseconds())
            .format($_('backoffice.song.durationFormat'));
    });

    function vote() {
        queuesLike(queueId, songId);
    }
    function remove() {
        // TODO
    }
    function ban() {
        // TODO
    }
    function next() {
        queuesNext(queueId, {
            song: songId
        });
    }

    const [Thumbnail, TData] = $derived.by(() => {
        const songData = song.song;
        if (isString(songData)) {
            return [null, null];
        }
        const songDetails = songData.details;
        if (!songDetails) {
            return [null, null];
        }

        return songThumbnailData({ ...songData, details: songDetails });
    });

    const iconsSizes = {
        height: 24,
        width: 24
    };

    const IconVoted: Component<typeof iconsSizes & { color: string }> | null =
        $derived.by(() => {
            switch (song.user_likes) {
                case 0:
                    return null;
                case 1:
                    return IconVotedOnce;
                case 2:
                    return IconVotedTwice;
                default:
                    return IconVotedMany;
            }
        });
</script>

{#snippet property(name: string, value: string)}
    <li>
        <em>{name}:</em>
        {value}
    </li>
{/snippet}

{#snippet actionButton(
    aria_label: string,
    onclick: () => void,
    IconElement: Component<typeof iconsSizes>
)}
    <button
        aria-label={aria_label}
        {onclick}
        class=" cursor-pointer rounded-lg border-0 shadow-lg transition-all hover:bg-[#2e7d37] focus:outline-none focus:ring-4 focus:ring-[#379B46]/50"
    >
        <IconElement {...iconsSizes} />
    </button>
{/snippet}

<li
    class={[
        'mb-2 grid h-[99px] w-full justify-stretch',
        permissions.includes('LIKE_SONG')
            ? 'grid-cols-[99px_auto_175px]'
            : 'grid-cols-[99px_auto]'
    ]}
>
    {#if song && !isString(song.song)}
        <Thumbnail src={TData} class="place-self-center" />
        <div class=" overflow-y-hidden pl-4 pr-4">
            <MarqueeOnHover host="h3" class="pb-1 text-lg font-semibold">
                {song.song.title}
            </MarqueeOnHover>
            <ul class="flex flex-row gap-2 text-sm text-gray-600">
                {@render property(
                    $_('backoffice.song.duration'),
                    duration || ''
                )}
                {@render property(
                    $_('backoffice.song.likes'),
                    song.user_likes.toString()
                )}
            </ul>
        </div>
        {#if permissions.includes('LIKE_SONG')}
            <div class="row-span-2 flex flex-col items-center justify-evenly">
                <button
                    onclick={vote}
                    class="flex h-12 w-[175px] cursor-pointer items-center justify-center gap-[10px] rounded-lg border-0 bg-[#379B46] text-base font-medium leading-none tracking-normal text-white shadow-lg transition-all hover:bg-[#2e7d37] focus:outline-none focus:ring-4 focus:ring-[#379B46]/50"
                >
                    {$_('backoffice.queue.like')}
                    <IconMoveUp {...iconsSizes} />
                </button>
                <div
                    class="flex h-6 items-center justify-end pt-3 text-base font-light leading-snug tracking-wide"
                >
                    {#if song.user_likes}
                        <span>
                            {$_('backoffice.queue.liked.pre', { default: '' })}
                            <em class="not-italic text-green-600">
                                {song.user_likes}
                                {$_('backoffice.queue.liked.unit')}
                            </em>
                            {$_('backoffice.queue.liked.post', { default: '' })}
                        </span>
                        <IconVoted {...iconsSizes} color="#379b46" />
                    {/if}
                </div>
            </div>
        {/if}
        <div class="col-span-2 flex gap-2 p-2">
            {#if permissions.includes('REMOVE_SONG')}
                {@render actionButton('remove', remove, IconRemove)}
            {/if}
            {#if false && permissions.includes('BAN_SONG')}
                {@render actionButton('ban', ban, IconBan)}
            {/if}
            {#if permissions.includes('NEXT_SONG')}
                {@render actionButton('playNext', next, IconPlay)}
            {/if}
        </div>
    {:else}
        <span>{$_('backoffice.song.loading')}</span>
    {/if}
</li>
