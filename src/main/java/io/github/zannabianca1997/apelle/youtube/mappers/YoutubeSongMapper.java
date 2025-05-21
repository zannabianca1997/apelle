package io.github.zannabianca1997.apelle.youtube.mappers;

import java.net.MalformedURLException;
import java.net.URI;
import java.net.URISyntaxException;
import java.net.URL;
import java.util.Collection;
import java.util.EnumMap;
import java.util.Map;
import java.util.Objects;
import java.util.stream.Stream;

import org.apache.http.client.utils.URIBuilder;
import org.eclipse.microprofile.config.ConfigProvider;
import org.mapstruct.AfterMapping;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.MappingTarget;
import org.mapstruct.Named;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.common.dtos.ThumbnailQueryDto;
import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSearchResultDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeThumbnailsDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeSong;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeThumbnail;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeThumbnailSize;

@Mapper(config = MappersConfig.class)
public interface YoutubeSongMapper {

    @Mapping(source = "videoData.snippet.title", target = "name")
    @Mapping(source = "videoData.contentDetails.duration", target = "duration")
    @Mapping(source = "videoData.snippet.thumbnails", target = "thumbnails")
    YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto, YoutubeVideoDataDto videoData);

    @AfterMapping
    default void fixThumbnailsRefs(@MappingTarget final YoutubeSong song) {
        song.getThumbnails().forEach((size, thumbnail) -> thumbnail.setSong(song));
    }

    default Map<YoutubeThumbnailSize, YoutubeThumbnail> fromDto(final YoutubeThumbnailsDto value) {
        if (value == null) {
            return null;
        }

        final Map<YoutubeThumbnailSize, YoutubeThumbnail> map = new EnumMap<>(YoutubeThumbnailSize.class);

        if (value.getDefault_() != null) {
            map.put(YoutubeThumbnailSize.DEFAULT, fromDto(value.getDefault_(), YoutubeThumbnailSize.DEFAULT));
        }
        if (value.getMedium() != null) {
            map.put(YoutubeThumbnailSize.MEDIUM, fromDto(value.getMedium(), YoutubeThumbnailSize.MEDIUM));
        }
        if (value.getHigh() != null) {
            map.put(YoutubeThumbnailSize.HIGH, fromDto(value.getHigh(), YoutubeThumbnailSize.HIGH));
        }
        if (value.getStandard() != null) {
            map.put(YoutubeThumbnailSize.STANDARD, fromDto(value.getStandard(), YoutubeThumbnailSize.STANDARD));
        }
        if (value.getMaxres() != null) {
            map.put(YoutubeThumbnailSize.MAXRES, fromDto(value.getMaxres(), YoutubeThumbnailSize.MAXRES));
        }

        return map;
    }

    YoutubeThumbnail fromDto(YoutubeThumbnailsDto.Thumbnail thumbnail, YoutubeThumbnailSize size);

    @Mapping(source = "snippet.title", target = "name")
    @Mapping(source = "snippet.thumbnails", target = "thumbnails")
    @Mapping(source = "id.videoId", target = "url", qualifiedByName = "watchUrl")
    @Mapping(source = "id.videoId", target = "enqueueData", qualifiedByName = "toAddDto")
    SearchedSongQueryDto toSearchedDto(YoutubeSearchResultDto searchResultDto);

    default Collection<ThumbnailQueryDto> toSearchedDto(final YoutubeThumbnailsDto youtubeThumbnailsDto) {
        return Stream.of(
                youtubeThumbnailsDto.getDefault_(),
                youtubeThumbnailsDto.getHigh(),
                youtubeThumbnailsDto.getMaxres(),
                youtubeThumbnailsDto.getMedium(),
                youtubeThumbnailsDto.getStandard()).filter(Objects::nonNull).map(this::toSearchedDto).toList();
    }

    ThumbnailQueryDto toSearchedDto(YoutubeThumbnailsDto.Thumbnail thumbnail);

    @Named("watchUrl")
    default URL watchURL(final String videoId) {
        try {
            return new URIBuilder(
                    ConfigProvider.getConfig()
                            .getValue("apelle.songs.sources.youtube.watch-uri", URI.class))
                    .addParameter("v", videoId)
                    .build().toURL();
        } catch (final URISyntaxException | MalformedURLException e) {
            throw new RuntimeException("The youtube url should always form a valid url", e);
        }
    }

    @Named("toAddDto")
    default YoutubeSongAddDto toAddDto(final String videoId) {
        return YoutubeSongAddDto.builder().videoId(videoId).build();
    }
}
