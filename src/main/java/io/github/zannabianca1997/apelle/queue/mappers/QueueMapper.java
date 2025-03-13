package io.github.zannabianca1997.apelle.queue.mappers;

import org.mapstruct.Mapper;

import io.github.zannabianca1997.apelle.queue.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queue.models.Queue;

@Mapper(componentModel = "cdi", uses = { SongMapper.class })
public interface QueueMapper {
    QueueQueryDto toDto(Queue queue);
}
