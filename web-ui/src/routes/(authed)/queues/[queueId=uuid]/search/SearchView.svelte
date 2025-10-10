<script lang="ts">
    import {
        songsSearch,
        type PaginatedSearchResponseItemCursor,
        type SearchResponseItem
    } from '$lib/apis/apelle';
    import type { Snapshot } from '@sveltejs/kit';
    import SearchBar from '$lib/components/forms/SearchBar.svelte';
    import { _ } from 'svelte-i18n';
    import IconExit from '~icons/mdi/close';
    import IconNextPage from '~icons/mdi/chevron-right';
    import IconPreviousPage from '~icons/mdi/chevron-left';
    import SearchedSongCard from './SearchedSongCard.svelte';
    import config from '$lib/config';

    const page_size = config.search.page_size;

    const {
        onSongChosen: onSongChosenInner,
        onDismiss: onDismissInner
    }: {
        onSongChosen?: (s: SearchResponseItem) => void;
        onDismiss?: () => void;
    } = $props();

    let query = $state('');
    let songs = $state.raw<PaginatedSearchResponseItemCursor | null>(null);
    let searching = $state(false);

    /**
     * Search for a given song.
     *
     * The promise will be resolved when the search is completed.
     * The return value will be true if the search is successfull
     *
     * @param q The song to search for
     */
    export async function searchFor(q: string): Promise<boolean> {
        searching = true;
        query = q;
        songs = (await songsSearch({ q: query, page_size })).data;
        searching = false;
        return true;
    }

    async function goto(page?: string | null) {
        if (!page) {
            return;
        }

        searching = true;
        songs = (await songsSearch({ q: query, page, page_size })).data;
        searching = false;
    }

    function onSongChosen(s: SearchResponseItem) {
        query = '';
        songs = null;
        searching = false;
        onSongChosenInner?.(s);
    }

    export const snapshot: Snapshot<{
        query: string;
        songs: PaginatedSearchResponseItemCursor;
    } | null> = {
        capture: () => songs && { query, songs },
        restore: (v) => {
            if (v) {
                query = v.query;
                songs = v.songs;
            }
        }
    };

    const onDismiss =
        onDismissInner &&
        (() => {
            query = '';
            songs = null;
            searching = false;
            onDismissInner();
        });
</script>

<section>
    <form onsubmit={() => searchFor(query)}>
        <SearchBar
            label={$_('backoffice.search.label')}
            submitTxt={$_('backoffice.search.submit')}
            bind:value={query}
        />
    </form>
</section>

<section>
    {#if songs}
        <table class="w-full table-fixed">
            <tbody>
                {#each songs.items as song}
                    <SearchedSongCard {song} {onSongChosen} />
                {/each}
            </tbody>
        </table>
    {/if}
</section>

<nav class="flex flex-row items-center gap-[10px]">
    {#if onDismiss}
        <button
            onclick={onDismiss}
            class="mr-auto flex cursor-pointer items-center justify-center border-0 bg-transparent p-0 text-white"
            aria-label={$_('backoffice.search.dismiss')}
        >
            <IconExit height={24} width={24} />
        </button>
    {/if}
    {#if songs}
        {#if songs.page_info.prev}
            <button
                onclick={() => goto(songs?.page_info.prev)}
                aria-label={$_('backoffice.search.prev')}
                class="flex cursor-pointer items-center justify-center border-0 bg-transparent p-0 text-white"
            >
                <IconPreviousPage height={24} width={24} />
            </button>
        {/if}
        <span>
            {$_('backoffice.search.pageOnTotal', {
                values: {
                    number: songs.page_info.page,
                    total:
                        songs.page_info.total != null
                            ? Math.ceil(
                                  songs.page_info.total / songs.page_info.size
                              )
                            : $_('backoffice.search.unknowPageNumber')
                }
            })}
        </span>
        {#if songs.page_info.next}
            <button
                onclick={() => goto(songs?.page_info.next)}
                aria-label={$_('backoffice.search.next')}
                class="flex cursor-pointer items-center justify-center border-0 bg-transparent p-0 text-white"
            >
                <IconNextPage height={24} width={24} />
            </button>
        {/if}
    {/if}
</nav>
