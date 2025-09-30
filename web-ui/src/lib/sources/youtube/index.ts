import type { PaginatedSearchResponseItemCursorItemsItemDetails } from '$lib/apis/apelle';
import { Logger } from '$lib/logger';
import type { SearchResultDetails, SourcePlugin } from '../types';

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

function isThumbnail(obj: any): obj is Thumbnail {
    return (
        typeof obj === 'object' &&
        obj !== null &&
        typeof obj.width === 'number' &&
        typeof obj.height === 'number' &&
        typeof obj.url === 'string'
    );
}

export function isSearchItemDetails(obj: unknown): obj is SearchItemDetails {
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
    }
} satisfies SourcePlugin;
