<script lang="ts">
    import {
        type PaginatedSearchResponseItemCursorItemsItem,
        type SearchResponseItem
    } from '$lib/apis/apelle';
    import { _ } from 'svelte-i18n';
    import IconAdd from '~icons/mdi/plus';
    import sources from '$lib/sources';
    import MarqueeOnHover from '$lib/components/MarqueeOnHover.svelte';
    import Button from '$lib/components/forms/Button.svelte';

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

<li class="grid w-full grid-cols-[99px_auto_100px] items-center">
    <Thumbnail src={TData} />
    <MarqueeOnHover class="m-[15px]">
        {details.title}
    </MarqueeOnHover>
    <Button icon={IconAdd} onclick={() => onSongChosen?.(song)} tight>
        {$_('backoffice.search.add')}
    </Button>
</li>
