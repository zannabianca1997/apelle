package io.github.zannabianca1997.apelle.youtube.mappers;

import java.util.EnumMap;
import java.util.Map;

import org.mapstruct.AfterMapping;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.MappingTarget;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
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
    default void fixThumbnailsRefs(@MappingTarget YoutubeSong song) {
        song.getThumbnails().forEach((size, thumbnail) -> thumbnail.setSong(song));
    }

    default Map<YoutubeThumbnailSize, YoutubeThumbnail> fromDto(YoutubeVideoDataDto.Snippet.Thumbnails value) {
        if (value == null) {
            return null;
        }

        Map<YoutubeThumbnailSize, YoutubeThumbnail> map = new EnumMap<>(YoutubeThumbnailSize.class);

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

    YoutubeThumbnail fromDto(YoutubeVideoDataDto.Snippet.Thumbnails.Thumbnail thumbnail, YoutubeThumbnailSize size);
}
