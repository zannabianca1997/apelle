package io.github.zannabianca1997.apelle.queues.roles.mappers;

import java.util.UUID;

import org.mapstruct.Mapper;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.roles.dtos.QueueUserRoleQueryDto;
import io.github.zannabianca1997.apelle.queues.roles.models.QueueUserRole;

@Mapper(config = MappersConfig.class)
public interface QueueUserRoleMapper {

    QueueUserRoleQueryDto toDto(QueueUserRole queueUserRole);

    default UUID toId(QueueUserRole queueUserRole) {
        if (queueUserRole == null) {
            return null;
        }
        return queueUserRole.getId();
    }
}
