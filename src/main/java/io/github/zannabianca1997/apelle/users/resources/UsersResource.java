package io.github.zannabianca1997.apelle.users.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.ResponseStatus;
import org.jboss.resteasy.reactive.RestResponse.StatusCode;

import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.exceptions.UserAlreadyExistsException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import jakarta.annotation.security.PermitAll;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;

@Path("/users")
@Tag(name = "Users", description = "User management")
public class UsersResource {

    @Inject
    UserMapper userMapper;
    @Inject
    UsersService usersService;
    @Inject
    UserResource userResource;

    @POST
    @PermitAll
    @Transactional
    @Operation(summary = "Create a user", description = "Create an user that can access queues and vote on them")
    @APIResponse(responseCode = "201", description = "The user created", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    @ResponseStatus(StatusCode.CREATED)
    public UserQueryDto signup(UserCreateDto userCreateDto) throws UserAlreadyExistsException {
        ApelleUser user = userMapper.createUser(userCreateDto);
        usersService.signup(user);
        return userMapper.toDto(user);
    }

    @Path("/me")
    public UserResource me() {
        return userResource.ofMe(usersService.getMe());
    }

    @Path("/n/{userName}")
    public UserResource byName(String userName) throws UserNotFoundByNameException {
        return userResource.of(usersService.getByName(userName));
    }

    @Path("/i/{userId}")
    public UserResource byId(UUID userId) throws UserNotFoundByIdException {
        return userResource.of(usersService.getById(userId));
    }
}
