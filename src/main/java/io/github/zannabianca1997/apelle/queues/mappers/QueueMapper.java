package io.github.zannabianca1997.apelle.queues.mappers;

import org.mapstruct.Mapper;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.models.Queue;

/**
 * Maps queues from dtos and back
 */
@Mapper(config = MappersConfig.class, uses = { SongMapper.class })
public interface QueueMapper {
    QueueQueryDto toDto(Queue queue);
}
