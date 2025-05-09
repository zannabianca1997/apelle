package io.github.zannabianca1997.apelle.queues.mappers;

import java.util.function.Function;

import org.mapstruct.Context;
import org.mapstruct.Mapper;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.roles.mappers.QueueUserRoleMapper;

/**
 * Maps queues from dtos and back
 */
@Mapper(config = MappersConfig.class, uses = { SongMapper.class, QueueUserRoleMapper.class })
public interface QueueMapper {
    QueueQueryDto toDto(Queue queue,
            @Context Function<QueuedSong, Short> getUserLikes);
}
