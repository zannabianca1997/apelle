package io.github.zannabianca1997.apelle.queue.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.queue.dtos.PlayingSongQueryDto;
import io.github.zannabianca1997.apelle.queue.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queue.dtos.SongQueryDto;
import io.github.zannabianca1997.apelle.queue.models.PlayingSong;
import io.github.zannabianca1997.apelle.queue.models.QueuedSong;
import io.github.zannabianca1997.apelle.queue.models.Song;

@Mapper(componentModel = "cdi")
public interface SongMapper {
    SongQueryDto toDto(Song song);

    @Mapping(source = "link.song", target = ".")
    QueuedSongQueryDto toDto(QueuedSong queuedSong);

    @Mapping(source = "song", target = ".")
    PlayingSongQueryDto toDto(PlayingSong playingSong);
}
