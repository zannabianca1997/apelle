package io.github.zannabianca1997.apelle.users.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;

import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;

/**
 * Maps users from and to DTOs
 */
@Mapper(componentModel = "cdi")
public interface UserMapper {
    UserQueryDto toDto(ApelleUser apelleUser);

    @Mapping(target = "roles", constant = "user")
    ApelleUser createUser(UserCreateDto userCreateDto);
}
