import type { PaginatedSearchResponseItemCursorItemsItemDetails } from '$lib/apis/apelle';
import type { Component } from 'svelte';
import type { HTMLImgAttributes } from 'svelte/elements';

export type SearchResultDetails = {
    title: string;
};

interface Thumbnail {
    width: number;
    height: number;
    url: string;
}
export type ThumbnailElement<TData> = Component<{ srcs: TData } & Omit<HTMLImgAttributes, 'src'>>

export interface SourcePlugin<TData> {
    searchDetails(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): SearchResultDetails;

    readonly ThumbnailElement: ThumbnailElement<TData>;
    thumbnailData(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): TData;
}
