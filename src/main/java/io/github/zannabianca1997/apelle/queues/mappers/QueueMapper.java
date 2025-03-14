package io.github.zannabianca1997.apelle.queues.mappers;

import java.net.MalformedURLException;

import org.mapstruct.Mapper;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.models.Queue;

/**
 * Maps queues from dtos and back
 */
@Mapper(componentModel = "cdi", uses = { SongMapper.class })
public interface QueueMapper {
    QueueQueryDto toDto(Queue queue) throws MalformedURLException;
}
