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
import jakarta.transaction.Transactional;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;

@Path("/users")
@Tag(name = "Users", description = "User management")
public class UsersResource {
    private final UserMapper userMapper;
    private final UsersService usersService;
    private final UserResource userResource;

    public UsersResource(final UserMapper userMapper, final UsersService usersService,
            final UserResource userResource) {
        this.userMapper = userMapper;
        this.usersService = usersService;
        this.userResource = userResource;
    }

    @POST
    @PermitAll
    @Transactional
    @Operation(summary = "Create a user", description = "Create an user that can access queues and vote on them")
    @APIResponse(responseCode = "201", description = "The user created", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    @ResponseStatus(StatusCode.CREATED)
    public UserQueryDto signup(final UserCreateDto userCreateDto) throws UserAlreadyExistsException {
        final ApelleUser user = userMapper.createUser(userCreateDto);
        usersService.signup(user);
        return userMapper.toDto(user);
    }

    @Path("/me")
    public UserResource me() {
        return userResource.of(usersService.getMe());
    }

    @Path("/n/{userName}")
    public UserResource byName(final String userName) throws UserNotFoundByNameException {
        return userResource.of(usersService.getByName(userName));
    }

    @Path("/i/{userId}")
    public UserResource byId(final UUID userId) throws UserNotFoundByIdException {
        return userResource.of(usersService.getById(userId));
    }
}
