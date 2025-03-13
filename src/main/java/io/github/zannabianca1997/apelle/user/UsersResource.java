package io.github.zannabianca1997.apelle.user;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;

import io.github.zannabianca1997.apelle.user.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.user.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.user.exceptions.UserAlreadyExistsException;
import io.github.zannabianca1997.apelle.user.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.user.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.user.mappers.UserMapper;
import io.github.zannabianca1997.apelle.user.models.ApelleUser;
import io.quarkus.security.Authenticated;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.annotation.security.PermitAll;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.Response.Status;

@Path("/users")
@Tag(name = "Users", description = "User management")
public class UsersResource {

    @Inject
    UserMapper userMapper;

    @POST
    @PermitAll
    @Transactional
    @Operation(summary = "Create a user", description = "Create an user that can access queues and vote on them")
    @APIResponse(responseCode = "201", description = "The user created", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public RestResponse<UserQueryDto> signup(UserCreateDto userCreateDto) throws UserAlreadyExistsException {
        if (ApelleUser.findByName(userCreateDto.getName()) != null) {
            throw new UserAlreadyExistsException(userCreateDto.getName());
        }
        ApelleUser user = userMapper.createUser(userCreateDto);
        user.persist();
        return RestResponse.<UserQueryDto>status(Status.CREATED, userMapper.toDto(user));
    }

    @GET
    @Path("/n/{userName}")
    @Authenticated
    @Operation(summary = "Find user by name", description = "Find a user by their username")
    @APIResponse(responseCode = "200", description = "The user was found", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto byName(String userName) throws UserNotFoundByNameException {
        ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            throw new UserNotFoundByNameException(userName);
        }
        return userMapper.toDto(user);
    }

    @GET
    @Path("/i/{userId}")
    @Authenticated
    @Operation(summary = "Find user by id", description = "Find a user by their universal id")
    @APIResponse(responseCode = "200", description = "The user was found", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto byName(UUID userId) throws UserNotFoundByIdException {
        ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            throw new UserNotFoundByIdException(userId);
        }
        return userMapper.toDto(user);
    }

    @GET
    @Path("/me")
    @Authenticated
    @Operation(summary = "Current user data", description = "Returns the data of the user that is currently logged in")
    @APIResponse(responseCode = "200", description = "The current user", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto me(@Context SecurityIdentity securityIdentity) {
        String name = securityIdentity.getPrincipal().getName();
        ApelleUser user = ApelleUser.findByName(name);
        return userMapper.toDto(user);
    }
}
