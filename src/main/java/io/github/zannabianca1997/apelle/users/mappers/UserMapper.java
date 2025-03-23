package io.github.zannabianca1997.apelle.users.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.common.configs.MappersConfig;
import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;
import jakarta.inject.Named;

/**
 * Maps users from and to DTOs
 */
@Mapper(config = MappersConfig.class, imports = { ApelleUserRole.class })
public interface UserMapper {
    @Named("generateUserDto")
    UserQueryDto toDto(ApelleUser apelleUser);

    @Mapping(target = "role", ignore = true)
    @Mapping(target = "roles", expression = "java( Set.of( ApelleUserRole.USER ) )")
    ApelleUser createUser(UserCreateDto userCreateDto);
}
