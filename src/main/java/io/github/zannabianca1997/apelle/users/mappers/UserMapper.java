package io.github.zannabianca1997.apelle.users.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.MappersConfig;
import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;

/**
 * Maps users from and to DTOs
 */
@Mapper(config = MappersConfig.class, imports = { ApelleUserRole.class })
public interface UserMapper {
    UserQueryDto toDto(ApelleUser apelleUser);

    @Mapping(target = "role", ignore = true)
    @Mapping(target = "roles", expression = "java( Set.of( ApelleUserRole.USER ) )")
    ApelleUser createUser(UserCreateDto userCreateDto);
}
