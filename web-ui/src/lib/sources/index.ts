import type {
    PaginatedSearchResponseItemCursorItemsItem,
    Song,
    SongDetailsAnyOf
} from '$lib/apis/apelle';
import { Logger } from '$lib/logger';
import type { Component } from 'svelte';
import type { SearchResultDetails, SourcePlugin } from './types';
import Youtube from './youtube';

const logger = new Logger('lib.sources');

function plugin(source: string): SourcePlugin<any> {
    switch (source) {
        case 'urn:apelle:sources/youtube':
            return Youtube;
        default:
            const msg = `Unknow source ${source}`;
            logger.error(msg);
            throw new Error(msg);
    }
}

export const searchDetails = (
    item: PaginatedSearchResponseItemCursorItemsItem
): SearchResultDetails => {
    return plugin(item.source).searchDetails(item.details);
};

export const searchThumbnailData = (
    item: PaginatedSearchResponseItemCursorItemsItem
): [Component<any>, any] => {
    const plugin_ = plugin(item.source);
    return [
        plugin_.ThumbnailElement,
        plugin_.searchThumbnailData(item.details)
    ];
};

export const songThumbnailData = (
    song: Song & { details: SongDetailsAnyOf }
): [Component<any>, any] => {
    const plugin_ = plugin(song.source);
    return [plugin_.ThumbnailElement, plugin_.songThumbnailData(song)];
};
