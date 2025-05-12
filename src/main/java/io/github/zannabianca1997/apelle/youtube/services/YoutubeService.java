package io.github.zannabianca1997.apelle.youtube.services;

import java.util.ArrayList;
import java.util.stream.Collectors;

import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.jboss.logging.Logger;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.ws.rs.BadRequestException;
import lombok.NonNull;
import io.github.zannabianca1997.apelle.common.dtos.Page;
import io.github.zannabianca1997.apelle.common.dtos.PageInfo;
import io.github.zannabianca1997.apelle.common.dtos.PageRequest;
import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.github.zannabianca1997.apelle.youtube.clients.YoutubeApiClient;
import io.github.zannabianca1997.apelle.youtube.configs.SearchConfig;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSearchResultDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.YoutubeVideoNotFoundException;
import io.github.zannabianca1997.apelle.youtube.mappers.YoutubeSongMapper;
import io.github.zannabianca1997.apelle.youtube.models.CachedYoutubeSearch;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeSong;
import io.quarkus.redis.datasource.RedisDataSource;
import io.quarkus.redis.datasource.value.SetArgs;
import io.quarkus.redis.datasource.value.ValueCommands;

@ApplicationScoped
public class YoutubeService {

    private final YoutubeApiClient youtubeApiVideosClient;
    private final YoutubeSongMapper songMapper;
    private final Logger log;
    private final ValueCommands<String, CachedYoutubeSearch> redisDataSource;
    private final SearchConfig searchConfig;

    public YoutubeService(
            @RestClient final YoutubeApiClient youtubeApiVideosClient,
            final YoutubeSongMapper songMapper,
            final Logger log,
            final RedisDataSource redisDataSource,
            final SearchConfig searchConfig) {
        this.youtubeApiVideosClient = youtubeApiVideosClient;
        this.songMapper = songMapper;
        this.log = log;
        this.redisDataSource = redisDataSource.value(CachedYoutubeSearch.class);
        this.searchConfig = searchConfig;
    }

    private YoutubeVideoDataDto getVideoData(final String videoId)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        final var videos = youtubeApiVideosClient.getDataById(videoId);
        if (videos.getItems().size() > 1) {
            throw new BadYoutubeApiResponseException("Multiple videos returned for a single id");
        }
        if (videos.getItems().isEmpty()) {
            throw new YoutubeVideoNotFoundException(videoId);
        }
        return videos.unwrapSingle();
    }

    /**
     * Complete the definition of a song by querying the youtube API
     * 
     * @param youtubeSongAddDto The data provided by the user
     * @return The completed song entity
     * @throws BadYoutubeApiResponseException An error happened while talking to
     *                                        youtube
     * @throws YoutubeVideoNotFoundException
     */
    public YoutubeSong fromDto(final YoutubeSongAddDto youtubeSongAddDto)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        // Try to obtain it from the database
        final YoutubeSong cached = YoutubeSong.findByVideoId(youtubeSongAddDto.getVideoId());
        if (cached != null) {
            return cached;
        }

        // Failed: asking the youtube gods
        final var videoData = getVideoData(youtubeSongAddDto.getVideoId());
        return songMapper.fromDto(youtubeSongAddDto, videoData);
    }

    /**
     * Normalize a query to improve cache reuse
     */
    private String normalizeQuery(final String query) {
        // TODO: trim the query to a maximum lenght
        return query
                .trim()
                .replace("\n", " ")
                .replaceAll("\\s+", " ")
                .toLowerCase();
    }

    /**
     * Calculate an unique key for a search.
     * 
     * The key is namespaced with the class name,
     */
    private String redisKey(final String query) {
        return getClass().getName() + ":search:" + query;
    }

    public Page<SearchedSongQueryDto> search(@NonNull String query, @NonNull final PageRequest pageRequest) {
        query = normalizeQuery(query);
        final var key = redisKey(query);

        boolean shouldSaveAtTheEnd = false;

        // Calculate page limits
        final int page = pageRequest.getPage() != null
                ? pageTokenToPageNumber(pageRequest.getPage())
                : 0;

        final int pageStart = page * pageRequest.getPageSize();
        final int pageEnd = pageStart + pageRequest.getPageSize();

        // Fetch the search from the redis cache if present
        var cached = redisDataSource.get(key);

        final boolean isFirstRequest = cached == null;

        // Check the youtube query limit
        final var currentNumberOfItems = cached != null ? cached.getFound().size() : 0;
        if (pageEnd > currentNumberOfItems
                && (pageEnd - currentNumberOfItems) > searchConfig.queriesAllowedPerRequest()
                        * searchConfig.pageSize()) {
            // TODO: change in a application exception
            throw new BadRequestException(
                    "Page would require too many youtube requests. Ask for a nearer page first");
        }

        // Start the search if not done before
        if (cached == null) {
            log.debugf("Starting a new search for `%s`", query);
            final var firstPage = youtubeApiVideosClient.getSearchByKeywords(searchConfig.pageSize(), query);

            cached = new CachedYoutubeSearch(
                    firstPage.getItems().stream().filter(YoutubeSearchResultDto::isVideo).map(songMapper::toSearchedDto)
                            .collect(Collectors.toCollection(() -> new ArrayList<>(searchConfig.pageSize()))),
                    firstPage.getPageInfo().getTotalResults(),
                    firstPage.getNextPageToken());

            shouldSaveAtTheEnd = true;
        }

        // Call the youtube api until the page is covered or the results end
        while (pageEnd > cached.getFound().size() && cached.getNextYoutubePageToken() != null) {
            log.debugf("Requesting a new page for `%s`", query);
            final var nextPage = youtubeApiVideosClient.getSearchPage(searchConfig.pageSize(), query,
                    cached.getNextYoutubePageToken());

            cached.getFound().addAll(nextPage.getItems().stream().filter(YoutubeSearchResultDto::isVideo)
                    .map(songMapper::toSearchedDto).toList());
            cached.setNextYoutubePageToken(nextPage.getNextPageToken());

            shouldSaveAtTheEnd = true;
        }

        // Cache the search
        if (shouldSaveAtTheEnd) {
            redisDataSource.set(key, cached,
                    isFirstRequest
                            // First request, set and set the expiration
                            ? new SetArgs().nx().ex(searchConfig.cacheExpiration())
                            // Not first request, just set and keep the expiration
                            : new SetArgs().xx().keepttl());

        }

        final var pageItems = cached.getFound().subList(Integer.min(pageStart, cached.getFound().size()),
                Integer.min(pageEnd, cached.getFound().size()));

        return Page.<SearchedSongQueryDto>builder()
                .items(pageItems)
                .pageInfo(PageInfo.builder()
                        .totalItems(cached.getTotalResults())
                        .items(pageItems.size())
                        .number(page)
                        .next(pageEnd >= cached.getTotalResults() ? null : pageNumberToPageToken(page + 1))
                        .prev(page == 0 ? null : pageNumberToPageToken(page - 1))
                        .build())
                .build();
    }

    private String pageNumberToPageToken(final int pageNumber) {
        return Integer.toString(pageNumber, 16);
    }

    private int pageTokenToPageNumber(final String pageToken) {
        try {
            return Integer.parseUnsignedInt(pageToken, 16);
        } catch (final NumberFormatException e) {
            // TODO: change in a application exception
            throw new BadRequestException("Invalid page token");
        }
    }

}
