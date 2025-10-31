import type {
    PaginatedSearchResponseItemCursorItemsItemDetails,
    Song,
    SongDetailsAnyOf
} from '$lib/apis/apelle';
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

function isThumbnail(obj: unknown): obj is Thumbnail {
    return (
        typeof obj === 'object' &&
        obj !== null &&
        'width' in obj &&
        typeof obj.width === 'number' &&
        'height' in obj &&
        typeof obj.height === 'number' &&
        'url' in obj &&
        typeof obj.url === 'string'
    );
}

function isSearchItemDetails(obj: unknown): obj is SearchItemDetails {
    if (typeof obj !== 'object' || obj === null) {
        return false;
    }

    const hasValidTitle = 'title' in obj && typeof obj.title === 'string';
    const hasValidUrl = 'url' in obj && typeof obj.url === 'string';

    if (!hasValidTitle || !hasValidUrl) {
        return false;
    }

    const hasValidThumbnailsArray =
        'thumbnails' in obj &&
        Array.isArray(obj.thumbnails) &&
        obj.thumbnails.every(isThumbnail);

    if (!hasValidThumbnailsArray) {
        return false;
    }

    return true;
}

function isYoutubeSongData(obj: unknown): obj is YoutubeSongData {
    if (typeof obj !== 'object' || obj === null) {
        return false;
    }

    const hasValidVideoId =
        'video_id' in obj && typeof obj.video_id === 'string';
    const hasValidUrl = 'url' in obj && typeof obj.url === 'string';

    if (!hasValidVideoId || !hasValidUrl) {
        return false;
    }

    const hasValidThumbsArray =
        'thumbs' in obj &&
        Array.isArray(obj.thumbs) &&
        obj.thumbs.every(isThumbnail);

    if (!hasValidThumbsArray) {
        return false;
    }

    return true;
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
    songThumbnailData(song: Song & { details: SongDetailsAnyOf }): Thumbnail[] {
        if (!isYoutubeSongData(song.details)) {
            const msg = 'Invalid value returned from Youtube provider';
            logger.error(msg, song.details);
            throw new Error(msg);
        }
        return Object.values(song.details.thumbs);
    }
} satisfies SourcePlugin<Thumbnail[]>;
