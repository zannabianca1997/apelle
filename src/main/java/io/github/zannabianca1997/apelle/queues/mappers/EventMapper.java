package io.github.zannabianca1997.apelle.queues.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.SubclassExhaustiveStrategy;
import org.mapstruct.SubclassMapping;

import io.github.zannabianca1997.apelle.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.websocket.server.QueueStateMessage;
import io.github.zannabianca1997.apelle.queues.dtos.websocket.server.ServerMessage;
import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueLikeEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueuePlayEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;

@Mapper(config = MappersConfig.class, subclassExhaustiveStrategy = SubclassExhaustiveStrategy.RUNTIME_EXCEPTION)
public interface EventMapper {
    @SubclassMapping(source = QueuePlayEvent.class, target = QueueStateMessage.class)
    @SubclassMapping(source = QueueStopEvent.class, target = QueueStateMessage.class)
    @SubclassMapping(source = QueueNextEvent.class, target = QueueStateMessage.class)
    @SubclassMapping(source = QueueLikeEvent.class, target = QueueStateMessage.class)
    @SubclassMapping(source = QueueEnqueueEvent.class, target = QueueStateMessage.class)
    ServerMessage toMessage(QueueEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateMessage toMessage(QueuePlayEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateMessage toMessage(QueueStopEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateMessage toMessage(QueueNextEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateMessage toMessage(QueueEnqueueEvent event);

    @Mapping(target = "queue", source = "state")
    QueueStateMessage toMessage(QueueLikeEvent event);
}
