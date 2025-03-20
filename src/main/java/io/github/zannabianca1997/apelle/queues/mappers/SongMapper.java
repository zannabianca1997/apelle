package io.github.zannabianca1997.apelle.queues.mappers;

import java.net.MalformedURLException;
import java.net.URI;
import java.net.URL;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.CurrentSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.sources.youtube.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.queues.models.CurrentSong;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.models.sources.youtube.YoutubeSong;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto;
import jakarta.inject.Named;

/**
 * Maps songs to dtos and back
 */
@Mapper(config = MappersConfig.class)
public abstract class SongMapper {
    @Mapping(source = "uri", target = "url")
    public abstract SongQueryDto toDto(Song song) throws MalformedURLException;

    @Mapping(source = "song", target = ".")
    @Mapping(source = "song.uri", target = "url")
    public abstract QueuedSongQueryDto toDto(QueuedSong queuedSong) throws MalformedURLException;

    @Mapping(source = "song", target = ".")
    @Mapping(source = "song.uri", target = "url")
    public abstract CurrentSongQueryDto toDto(CurrentSong playingSong) throws MalformedURLException;

    public YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto, VideoDataDto videoData) {
        if (youtubeSongAddDto == null || videoData == null) {
            return null;
        }

        return fromDtoInner(youtubeSongAddDto, videoData);
    }

    @Named("ignore")
    @Mapping(source = "videoData.snippet.title", target = "name")
    @Mapping(source = "videoData.contentDetails.duration", target = "duration")
    protected abstract YoutubeSong fromDtoInner(YoutubeSongAddDto youtubeSongAddDto, VideoDataDto videoData);

    protected URL toUrl(URI uri) throws MalformedURLException {
        return uri.toURL();
    }
}
