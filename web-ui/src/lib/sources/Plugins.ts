import type {
    PaginatedSearchResponseItemCursorItemsItem,
    Song,
    SongDetailsAnyOf
} from '$lib/apis/apelle';
import { Logger } from '$lib/logger';
import type { Component } from 'svelte';
import type {
    PlayerElement,
    SearchResultDetails,
    SourcePlugin,
    ThumbnailElement
} from './types';

const logger = new Logger('lib.sources.api');

type ThumbnailElementAndData<Plugin> =
    Plugin extends SourcePlugin<infer TData, any>
        ? [ThumbnailElement<TData>, TData]
        : never;

type GenericSourcePlugin = SourcePlugin<any, any>;

export default class Plugins<PluginIndex> {
    constructor(
        private readonly plugins: PluginIndex & {
            [key: string]: GenericSourcePlugin;
        }
    ) {}

    private plugin(source: string): GenericSourcePlugin {
        if (source in this.plugins) {
            return this.plugins[source];
        }

        const msg = `Unknow source ${source}`;
        logger.error(msg);
        throw new Error(msg);
    }

    public searchDetails(
        item: PaginatedSearchResponseItemCursorItemsItem
    ): SearchResultDetails {
        return this.plugin(item.source).searchDetails(item.details);
    }

    public searchThumbnailData(
        item: PaginatedSearchResponseItemCursorItemsItem
    ): ThumbnailElementAndData<GenericSourcePlugin> {
        const plugin = this.plugin(item.source);
        return [
            plugin.ThumbnailElement,
            plugin.searchThumbnailData(item.details)
        ];
    }

    public songThumbnailData(
        song: Song & { details: SongDetailsAnyOf }
    ): ThumbnailElementAndData<GenericSourcePlugin> {
        const plugin = this.plugin(song.source);
        return [plugin.ThumbnailElement, plugin.songThumbnailData(song)];
    }

    public playerElement(
        song: Song & { details: SongDetailsAnyOf }
    ): PlayerElement<any> {
        const plugin = this.plugin(song.source);
        return plugin.PlayerElement;
    }
}
