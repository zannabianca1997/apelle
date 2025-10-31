<script lang="ts">
    import {
        type PaginatedSearchResponseItemCursorItemsItem,
        type SearchResponseItem
    } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    import IconAdd from '~icons/mdi/plus';
    import sources from '$lib/sources';
    import MarqueeOnHover from '$lib/components/MarqueeOnHover.svelte';

    const {
        song,
        onSongChosen
    }: {
        song: PaginatedSearchResponseItemCursorItemsItem;
        onSongChosen?: (s: SearchResponseItem) => void;
    } = $props();

    const details = $derived(sources.searchDetails(song));
    const [Thumbnail, TData] = $derived(sources.searchThumbnailData(song));
</script>

<tr>
    <td class="h-[99px] w-[176px] bg-transparent p-2">
        <Thumbnail src={TData} class="h-full w-full" />
    </td>
    <MarqueeOnHover host="td" class="pl-[15px]">
        {details.title}
    </MarqueeOnHover>
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
