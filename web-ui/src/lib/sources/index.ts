import type { PaginatedSearchResponseItemCursorItemsItem } from '$lib/apis/apelle';
import { Logger } from '$lib/logger';
import type { SearchResultDetails, SourcePlugin } from './types';
import Youtube from './youtube';

const logger = new Logger('lib.sources');

function plugin(source: string): SourcePlugin {
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
