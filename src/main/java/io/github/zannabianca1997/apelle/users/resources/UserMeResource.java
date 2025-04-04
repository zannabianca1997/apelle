package io.github.zannabianca1997.apelle.users.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

@Path("/users/me")
@Tag(name = "Users", description = "User management")
public class UserMeResource {

    @Inject
    UserMapper userMapper;
    @Inject
    UsersService usersService;

    @GET
    @Authenticated
    @Operation(summary = "Current user data", description = "Returns the data of the user that is currently logged in")
    @APIResponse(responseCode = "200", description = "The current user", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto get() {
        return userMapper.toDto(usersService.getCurrent());
    }

    @DELETE
    @Authenticated
    @Transactional
    @Operation(summary = "Delete current user", description = "Delete the current user")
    @APIResponse(responseCode = "200", description = "The current user was deleted")
    public void delete() {
        usersService.deleteCurrent();
    }
}
