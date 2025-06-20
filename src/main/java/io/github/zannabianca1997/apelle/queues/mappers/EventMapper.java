package io.github.zannabianca1997.apelle.queues.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.SubclassExhaustiveStrategy;
import org.mapstruct.SubclassMapping;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueStateEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueuedSongDeleteEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueuedSongsStateEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.CurrentSongStateEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueDeleteEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueEventDto;
import io.github.zannabianca1997.apelle.queues.events.QueueDeleteEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueLikeEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStartEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;
import io.github.zannabianca1997.apelle.queues.events.QueuedSongDeleteEvent;

@Mapper(config = MappersConfig.class, subclassExhaustiveStrategy = SubclassExhaustiveStrategy.RUNTIME_EXCEPTION)
public interface EventMapper {
    @SubclassMapping(source = QueueStartEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueStopEvent.class, target = CurrentSongStateEventDto.class)
    @SubclassMapping(source = QueueNextEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueLikeEvent.class, target = QueuedSongsStateEventDto.class)
    @SubclassMapping(source = QueueEnqueueEvent.class, target = QueuedSongsStateEventDto.class)
    @SubclassMapping(source = QueueDeleteEvent.class, target = QueueDeleteEventDto.class)
    @SubclassMapping(source = QueuedSongDeleteEvent.class, target = QueuedSongDeleteEventDto.class)
    QueueEventDto toDto(QueueEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toDto(QueueStartEvent event);

    @Mapping(target = "current", source = "state")
    CurrentSongStateEventDto toDto(QueueStopEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toDto(QueueNextEvent event);

    QueuedSongsStateEventDto toDto(QueueEnqueueEvent event);

    QueuedSongsStateEventDto toDto(QueueLikeEvent event);

    QueueDeleteEventDto toDto(QueueDeleteEvent event);

    QueuedSongDeleteEventDto toDto(QueuedSongDeleteEvent event);
}
