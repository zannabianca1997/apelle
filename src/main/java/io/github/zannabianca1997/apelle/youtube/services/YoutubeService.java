package io.github.zannabianca1997.apelle.youtube.services;

import java.util.ArrayList;
import java.util.stream.Collectors;

import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.jboss.logging.Logger;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.ws.rs.BadRequestException;
import lombok.NonNull;
import io.github.zannabianca1997.apelle.common.models.Page;
import io.github.zannabianca1997.apelle.common.models.PageInfo;
import io.github.zannabianca1997.apelle.common.models.PageRequest;
import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.github.zannabianca1997.apelle.youtube.clients.YoutubeApiClient;
import io.github.zannabianca1997.apelle.youtube.configs.SearchConfig;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.YoutubeVideoNotFoundException;
import io.github.zannabianca1997.apelle.youtube.mappers.YoutubeSongMapper;
import io.github.zannabianca1997.apelle.youtube.models.CachedYoutubeSearch;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeSong;
import io.quarkus.redis.datasource.RedisDataSource;
import io.quarkus.redis.datasource.value.ValueCommands;

@ApplicationScoped
public class YoutubeService {

    @Inject
    @RestClient
    YoutubeApiClient youtubeApiVideosClient;

    @Inject
    YoutubeSongMapper songMapper;

    @Inject
    Logger log;

    private YoutubeVideoDataDto getVideoData(String videoId)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        var videos = youtubeApiVideosClient.getDataById(videoId);
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
    public YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        // Try to obtain it from the database
        YoutubeSong cached = YoutubeSong.findByVideoId(youtubeSongAddDto.getVideoId());
        if (cached != null) {
            return cached;
        }

        // Failed: asking the youtube gods
        var videoData = getVideoData(youtubeSongAddDto.getVideoId());
        return songMapper.fromDto(youtubeSongAddDto, videoData);
    }

    ValueCommands<String, CachedYoutubeSearch> redisDataSource;

    @Inject
    void setRedisCommand(RedisDataSource redisDataSource) {
        this.redisDataSource = redisDataSource.value(CachedYoutubeSearch.class);
    }

    /**
     * Normalize a query to improve cache reuse
     */
    private String normalizeQuery(String query) {
        // TODO: trim the query to a maximum lenght
        return query
                .trim()
                .replaceAll("\n", "")
                .replaceAll("\\s+", " ")
                .toLowerCase();
    }

    /**
     * Calculate an unique key for a search.
     * 
     * The key is namespaced with the class name,
     */
    private String redisKey(String query) {
        return getClass().getName() + ":search:" + query;
    }

    @Inject
    SearchConfig searchConfig;

    public Page<SearchedSongQueryDto> search(@NonNull String query, @NonNull PageRequest pageRequest) {
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

            cached = new CachedYoutubeSearch(firstPage.getItems().stream().map(songMapper::toSearchedDto)
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

            cached.getFound().addAll(nextPage.getItems().stream().map(songMapper::toSearchedDto).toList());
            cached.setNextYoutubePageToken(nextPage.getNextPageToken());

            shouldSaveAtTheEnd = true;
        }

        // Cache the search
        if (shouldSaveAtTheEnd) {
            redisDataSource.setex(key, searchConfig.cacheExpiration().toSeconds(), cached);
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

    private String pageNumberToPageToken(int pageNumber) {
        return Integer.toString(pageNumber, 16);
    }

    private int pageTokenToPageNumber(String pageToken) {
        try {
            return Integer.parseUnsignedInt(pageToken, 16);
        } catch (NumberFormatException e) {
            // TODO: change in a application exception
            throw new BadRequestException("Invalid page token");
        }
    }

}
