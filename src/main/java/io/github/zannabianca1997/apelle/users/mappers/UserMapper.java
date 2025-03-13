package io.github.zannabianca1997.apelle.users.mappers;

import org.mapstruct.Mapper;
import org.mapstruct.Mapping;
import org.mapstruct.Named;

import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.elytron.security.common.BcryptUtil;

@Mapper(componentModel = "cdi")
public interface UserMapper {
    UserQueryDto toDto(ApelleUser apelleUser);

    @Mapping(target = "id", ignore = true)
    @Mapping(target = "roles", constant = "user")
    @Mapping(target = "password", qualifiedByName = "hashPassword")
    ApelleUser createUser(UserCreateDto userCreateDto);

    @Named("hashPassword")
    default String hashPassword(String password) {
        return BcryptUtil.bcryptHash(password);
    }
}
