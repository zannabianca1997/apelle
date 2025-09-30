<script lang="ts">
    import {
        type PaginatedSearchResponseItemCursorItemsItem,
        type SearchResponseItem
    } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    // import Thumbnail from '../Thumbnail.svelte';
    import IconAdd from '~icons/mdi/plus';
    import { searchDetails } from '$lib/sources';

    const {
        song,
        onSongChosen
    }: {
        song: PaginatedSearchResponseItemCursorItemsItem;
        onSongChosen?: (s: SearchResponseItem) => void;
    } = $props();

    const details = $derived(searchDetails(song));
</script>

<tr>
    <!-- <td class="w-[176px] h-[99px] bg-transparent p-0">
                {#if song.thumbnails}
                    <Thumbnail thumbnails={song.thumbnails} />
                {/if}
            </td> -->
    <td class="overflow-hidden pl-[15px] text-ellipsis whitespace-nowrap">
        {details.title}
    </td>
    <td class="w-[175px]">
        <button
            onclick={() => onSongChosen?.(song)}
            class="flex h-12 w-full cursor-pointer items-center justify-center gap-[10px] rounded border-0 bg-[#3a3a3a] p-[6px] px-3 text-white"
        >
            {$_('backoffice.search.add')}
            <IconAdd height={24} width={24} />
        </button>
    </td>
</tr>
