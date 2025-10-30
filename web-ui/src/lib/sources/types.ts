import type {
    PaginatedSearchResponseItemCursorItemsItemDetails,
    Song,
    SongDetailsAnyOf
} from '$lib/apis/apelle';
import type { Component } from 'svelte';
import type { HTMLImgAttributes } from 'svelte/elements';

export type SearchResultDetails = {
    title: string;
};

export type ThumbnailElement<TData> = Component<
    { src: TData } & Omit<HTMLImgAttributes, 'src'>
>;

export interface SourcePlugin<TData> {
    searchDetails(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): SearchResultDetails;

    readonly ThumbnailElement: ThumbnailElement<TData>;
    searchThumbnailData(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): TData;
    songThumbnailData(song: Song & { details: SongDetailsAnyOf }): TData;
}
