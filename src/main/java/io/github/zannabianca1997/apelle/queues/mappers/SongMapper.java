package io.github.zannabianca1997.apelle.queues.mappers;

import java.net.MalformedURLException;
import java.net.URI;
import java.net.URL;
import java.util.function.Function;

import org.mapstruct.Context;
import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.CurrentSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongShortQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongQueryDto;
import io.github.zannabianca1997.apelle.queues.models.CurrentSong;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;

/**
 * Maps songs to dtos and back
 */
@Mapper(config = MappersConfig.class)
public interface SongMapper {
    @Mapping(source = "uri", target = "url")
    @Mapping(source = "allThumbnails", target = "thumbnails")
    SongQueryDto toDto(Song song);

    @Mapping(source = "queuedSong.likes", target = "likes")
    @Mapping(source = "queuedSong.song", target = ".")
    QueuedSongShortQueryDto toShortDto(QueuedSong queuedSong, short userLikes);

    default QueuedSongShortQueryDto toShortDto(final QueuedSong queuedSong,
            @Context final Function<QueuedSong, Short> getUserLikes) {
        return toShortDto(queuedSong, getUserLikes.apply(queuedSong));
    }

    @Mapping(source = "queuedSong.likes", target = "likes")
    @Mapping(source = "queuedSong.song", target = ".")
    @Mapping(source = "queuedSong.song.uri", target = "url")
    @Mapping(source = "queuedSong.song.allThumbnails", target = "thumbnails")
    QueuedSongQueryDto toDto(QueuedSong queuedSong, short userLikes);

    @Mapping(source = "song", target = ".")
    @Mapping(source = "song.uri", target = "url")
    @Mapping(source = "song.allThumbnails", target = "thumbnails")
    CurrentSongQueryDto toDto(CurrentSong currentSong);

    default URL toUrl(final URI uri) {
        try {
            return uri.toURL();
        } catch (final MalformedURLException e) {
            // This should not happen, as the url generated should always be valid
            throw new RuntimeException(e);
        }
    }
}
