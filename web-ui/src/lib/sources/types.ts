import type { PaginatedSearchResponseItemCursorItemsItemDetails } from '$lib/apis/apelle';

export type SearchResultDetails = {
    title: string;
};

export interface SourcePlugin {
    searchDetails(
        details: PaginatedSearchResponseItemCursorItemsItemDetails
    ): SearchResultDetails;
}
