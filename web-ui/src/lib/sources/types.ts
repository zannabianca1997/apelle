/**
 * Types relative to source plugins
 */

import type {
    PaginatedSearchResponseItemCursorItemsItemDetails,
    Song,
    SongDetailsAnyOf
} from '$lib/apis/apelle';
import type { durationjs } from '$lib/time';
import type { Component } from 'svelte';
import type { HTMLImgAttributes } from 'svelte/elements';

export type SearchResultDetails = {
    title: string;
};

export type ThumbnailElement<TData> = Component<
    { src: TData } & Omit<HTMLImgAttributes, 'src'>
>;

export type PlayerElement<SourceDetails> = Component<
    {
        song: Song & { details: SourceDetails },
        height?: number | string,
        width?: number | string,
        position: durationjs.Duration,
        /// if the song is stopped
        stopped: boolean,
        /// Volume, between 0 and 1
        volume: number
    }
>;

export interface SourcePlugin<TData, PDetails> {
    searchDetails(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): SearchResultDetails;

    readonly ThumbnailElement: ThumbnailElement<TData>;
    searchThumbnailData(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): TData;
    songThumbnailData(song: Song & { details: SongDetailsAnyOf }): TData;

    readonly PlayerElement: PlayerElement<PDetails>;
}
