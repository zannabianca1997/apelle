package io.github.zannabianca1997.apelle.users.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.exceptions.CannotDeleteUserException;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.quarkus.security.Authenticated;
import jakarta.annotation.security.PermitAll;
import jakarta.enterprise.context.Initialized;
import jakarta.enterprise.context.RequestScoped;
import jakarta.enterprise.event.Observes;
import jakarta.transaction.TransactionScoped;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.GET;

@RequestScoped
@Authenticated
public class UserResource {
    private final UserMapper userMapper;
    private final UsersService usersService;

    public UserResource(final UserMapper userMapper, final UsersService usersService) {
        this.userMapper = userMapper;
        this.usersService = usersService;
    }

    private ApelleUser user = null;

    public UserResource of(final ApelleUser user) {
        this.user = user;
        return this;
    }

    @PermitAll
    void onBeginTransaction(@Observes @Initialized(TransactionScoped.class) final Object event) {
        if (user != null)
            user = ApelleUser.getEntityManager().merge(user);
    }

    @GET
    @Operation(summary = "The user data", description = "Returns the data of the user")
    @APIResponse(responseCode = "200", description = "The user", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto get() {
        return userMapper.toDto(user);
    }

    @DELETE
    @Transactional
    @Operation(summary = "Delete user", description = "Delete the user")
    @APIResponse(responseCode = "200", description = "The user was deleted")
    public void delete() throws CannotDeleteUserException {
        usersService.delete(user);
    }
}
