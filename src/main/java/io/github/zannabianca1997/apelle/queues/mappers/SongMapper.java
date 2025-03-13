package io.github.zannabianca1997.apelle.queues.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.queues.dtos.PlayingSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongQueryDto;
import io.github.zannabianca1997.apelle.queues.models.PlayingSong;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;

@Mapper(componentModel = "cdi")
public interface SongMapper {
    SongQueryDto toDto(Song song);

    @Mapping(source = "link.song", target = ".")
    QueuedSongQueryDto toDto(QueuedSong queuedSong);

    @Mapping(source = "song", target = ".")
    PlayingSongQueryDto toDto(PlayingSong playingSong);
}
