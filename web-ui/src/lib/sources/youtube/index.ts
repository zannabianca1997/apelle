import type { PaginatedSearchResponseItemCursorItemsItemDetails, Song, SongDetailsAnyOf } from '$lib/apis/apelle';
import { Logger } from '$lib/logger';
import type { SearchResultDetails, SourcePlugin } from '../types';

import ThumbnailElement from '$lib/components/Thumbnail.svelte';

const logger = new Logger('lib.sources.youtube');

interface Thumbnail {
    width: number;
    height: number;
    url: string;
}

interface SearchItemDetails {
    title: string;
    url: string;
    thumbnails: Thumbnail[];
}

interface YoutubeSongData {
    video_id: string;
    url: string;
    thumbs: Thumbnail[];
}

function isThumbnail(obj: any): obj is Thumbnail {
    return (
        typeof obj === 'object' &&
        obj !== null &&
        typeof obj.width === 'number' &&
        typeof obj.height === 'number' &&
        typeof obj.url === 'string'
    );
}

function isSearchItemDetails(obj: unknown): obj is SearchItemDetails {
    if (typeof obj !== 'object' || obj === null) {
        return false;
    }

    const item = obj as any;

    const hasValidTitle = typeof item.title === 'string';
    const hasValidUrl = typeof item.url === 'string';

    if (!hasValidTitle || !hasValidUrl) {
        return false;
    }

    const hasValidThumbnailsArray = Array.isArray(item.thumbnails);

    if (!hasValidThumbnailsArray) {
        return false;
    }

    const allThumbnailsAreValid = item.thumbnails.every(isThumbnail);
    return allThumbnailsAreValid;
}

function isYoutubeSongData(obj: any): obj is YoutubeSongData {
    if (typeof obj !== 'object' || obj === null) {
        return false;
    }

    const item = obj as any;

    const hasValidVideoId = typeof item.video_id === 'string';
    const hasValidUrl = typeof item.url === 'string';

    if (!hasValidVideoId || !hasValidUrl) {
        return false;
    }

    const hasValidThumbsArray = Array.isArray(item.thumbs);

    if (!hasValidThumbsArray) {
        return false;
    }

    const allThumbnailsAreValid = item.thumbs.every(isThumbnail);
    return allThumbnailsAreValid;
}

export default {
    searchDetails(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): SearchResultDetails {
        if (!isSearchItemDetails(details)) {
            const msg = 'Invalid value returned from Youtube provider';
            logger.error(msg, details);
            throw new Error(msg);
        }
        return details;
    },

    ThumbnailElement: ThumbnailElement,
    searchThumbnailData(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): Thumbnail[] {
        if (!isSearchItemDetails(details)) {
            const msg = 'Invalid value returned from Youtube provider';
            logger.error(msg, details);
            throw new Error(msg);
        }
        return details.thumbnails;
    },
    songThumbnailData(
        song: Song & { details: SongDetailsAnyOf }
    ): Thumbnail[] {
        if (!isYoutubeSongData(song.details)) {
            const msg = 'Invalid value returned from Youtube provider';
            logger.error(msg, song.details);
            throw new Error(msg);
        }
        return Object.values(song.details.thumbs)
    }
} satisfies SourcePlugin<Thumbnail[]>;
