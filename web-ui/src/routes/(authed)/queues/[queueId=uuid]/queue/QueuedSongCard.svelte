<script lang="ts">
    import { queuesLike, songsGet, type QueuedSong } from '$lib/apis/apelle';
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
    const logger = new Logger(
        'routes.authed.queues.queueIdUuid.queue.QueuedSongCard'
    );

    let {
        queueId,
        song = $bindable()
    }: {
        queueId: string;
        song: QueuedSong;
    } = $props();

    function fetchData(id: string) {
        songsGet(id, {
            details: true
        }).then(({ data }) => (song.song = data));
    }

    $effect(() => {
        if (isString(song.song)) {
            fetchData(song.song);
        } else if (isString(song.song.details)) {
            fetchData(song.song.id);
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
        queuesLike(queueId, isString(song.song) ? song.song : song.song.id);
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
</script>

{#snippet property(name: string, value: string)}
    <li>
        <em>{name}:</em>
        {value}
    </li>
{/snippet}

<li class="grid h-[99px] w-full grid-cols-[99px_auto_175px] justify-stretch">
    {#if song && !isString(song.song)}
        <Thumbnail src={TData} class="place-self-center" />
        <div class="flex flex-col justify-center overflow-hidden pl-4 pr-4">
            <MarqueeOnHover host="h3" class="text-lg font-semibold">
                {song.song.title}
            </MarqueeOnHover>
            <ul class="text-sm text-gray-600">
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
        <div class="flex flex-col items-center">
            <button
                onclick={vote}
                class="flex h-12 w-[175px] cursor-pointer items-center justify-center gap-[10px] rounded-lg border-0 bg-[#379B46] text-base font-medium leading-none tracking-normal text-white shadow-lg transition-all hover:bg-[#2e7d37] focus:outline-none focus:ring-4 focus:ring-[#379B46]/50"
            >
                {$_('backoffice.queue.like')}
                <IconMoveUp height={24} width={24} />
            </button>
            <div
                class="flex h-6 items-center justify-end pt-3 text-base font-light leading-snug tracking-wide"
            >
                {#if song.user_likes && song.user_likes > 0}
                    <span>
                        {$_('backoffice.queue.liked.pre', { default: '' })}
                        <em class="not-italic text-green-600">
                            {song.user_likes}
                            {$_('backoffice.queue.liked.unit')}
                        </em>
                        {$_('backoffice.queue.liked.post', { default: '' })}
                    </span>
                    {#if song.user_likes === 1}
                        <IconVotedOnce height={24} width={24} color="#379b46" />
                    {:else if song.user_likes === 2}
                        <IconVotedTwice
                            height={24}
                            width={24}
                            color="#379b46"
                        />
                    {:else}
                        <IconVotedMany height={24} width={24} color="#379b46" />
                    {/if}
                {/if}
            </div>
        </div>
    {:else}
        <span>{$_('backoffice.song.loading')}</span>
    {/if}
</li>
