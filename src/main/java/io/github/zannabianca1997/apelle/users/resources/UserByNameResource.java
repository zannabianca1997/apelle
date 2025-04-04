package io.github.zannabianca1997.apelle.users.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.quarkus.security.Authenticated;
import jakarta.annotation.security.RolesAllowed;
import jakarta.inject.Inject;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

@Path("/users/n/{userName}")
@Tag(name = "Users", description = "User management")
public class UserByNameResource {

    @Inject
    UserMapper userMapper;
    @Inject
    UsersService usersService;

    @GET
    @Authenticated
    @Operation(summary = "Find user by name", description = "Find a user by their username")
    @APIResponse(responseCode = "200", description = "The user was found", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto get(String userName) throws UserNotFoundByNameException {
        return userMapper.toDto(usersService.get(userName));
    }

    @DELETE
    @RolesAllowed({ "ADMIN" })
    @Operation(summary = "Delete a user by name", description = "Delete a user by name. Need to have the role `admin`")
    @APIResponse(responseCode = "200", description = "The user was deleted")
    public void delete(String userName) throws UserNotFoundByNameException {
        usersService.delete(userName);
    }
}
