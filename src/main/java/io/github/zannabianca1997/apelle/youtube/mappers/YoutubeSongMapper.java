package io.github.zannabianca1997.apelle.youtube.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeSong;

@Mapper(config = MappersConfig.class)
public interface YoutubeSongMapper {

    @Mapping(source = "videoData.snippet.title", target = "name")
    @Mapping(source = "videoData.contentDetails.duration", target = "duration")
    YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto, YoutubeVideoDataDto videoData);

}
