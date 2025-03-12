package io.github.zannabianca1997.apelle.user;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;

import io.github.zannabianca1997.apelle.user.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.user.dtos.UserQueryDto;
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
import jakarta.ws.rs.core.Response;

@Path("/users")
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
    @APIResponse(responseCode = "400", description = "Malformed JSON")
    @APIResponse(responseCode = "409", description = "This user already exists")
    public Response signup(UserCreateDto userCreateDto) {
        if (ApelleUser.findByName(userCreateDto.getName()) != null) {
            return Response.status(Response.Status.CONFLICT).build();
        }
        ApelleUser user = userMapper.createUser(userCreateDto);
        user.persist();
        return Response.status(Response.Status.CREATED).entity(userMapper.toDto(user)).build();
    }

    @GET
    @Path("/n/{userName}")
    @Authenticated
    @Operation(summary = "Find user by name", description = "Find a user by their username")
    @APIResponse(responseCode = "200", description = "The user was found", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    @APIResponse(responseCode = "404", description = "The user does not exist")
    public Response byName(String userName) {
        ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            return Response.status(Response.Status.NOT_FOUND).build();
        }
        return Response.ok(userMapper.toDto(user)).build();
    }

    @GET
    @Path("/i/{userId}")
    @Authenticated
    @Operation(summary = "Find user by id", description = "Find a user by their universal id")
    @APIResponse(responseCode = "200", description = "The user was found", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    @APIResponse(responseCode = "404", description = "The user does not exist")
    public Response byName(UUID userId) {
        ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            return Response.status(Response.Status.NOT_FOUND).build();
        }
        return Response.ok(userMapper.toDto(user)).build();
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
