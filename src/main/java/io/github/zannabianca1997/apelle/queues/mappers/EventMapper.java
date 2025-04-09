package io.github.zannabianca1997.apelle.queues.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.SubclassExhaustiveStrategy;
import org.mapstruct.SubclassMapping;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueStateEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueDeleteEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueEventDto;
import io.github.zannabianca1997.apelle.queues.events.QueueDeleteEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueLikeEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStartEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;

@Mapper(config = MappersConfig.class, subclassExhaustiveStrategy = SubclassExhaustiveStrategy.RUNTIME_EXCEPTION)
public interface EventMapper {
    @SubclassMapping(source = QueueStartEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueStopEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueNextEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueLikeEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueEnqueueEvent.class, target = QueueStateEventDto.class)
    @SubclassMapping(source = QueueDeleteEvent.class, target = QueueDeleteEventDto.class)
    QueueEventDto toDto(QueueEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toMessage(QueueStartEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toMessage(QueueStopEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toMessage(QueueNextEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toMessage(QueueEnqueueEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateEventDto toMessage(QueueLikeEvent event);

    QueueDeleteEventDto toMessage(QueueDeleteEvent event);
}
