package io.github.zannabianca1997.apelle.queues.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.queues.dtos.QueueUserQueryDto;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.roles.mappers.QueueUserRoleMapper;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import jakarta.inject.Inject;

@Mapper(config = MappersConfig.class, uses = { QueueUserRoleMapper.class })
public abstract class QueueUserMapper {
    @Inject
    UserMapper userMapper;

    @Mapping(target = "queueRole", source = "queueUser.role")
    protected abstract QueueUserQueryDto toDtoInner(QueueUser queueUser, UserQueryDto userQueryDto);

    public QueueUserQueryDto toDto(final QueueUser queueUser) {
        if (queueUser == null) {
            return null;
        }
        return toDtoInner(queueUser, userMapper.toDto(queueUser.getUser()));
    }

}
