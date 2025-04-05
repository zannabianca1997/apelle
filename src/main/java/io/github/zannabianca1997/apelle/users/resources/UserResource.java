package io.github.zannabianca1997.apelle.users.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.quarkus.security.Authenticated;
import io.quarkus.security.PermissionChecker;
import io.quarkus.security.PermissionsAllowed;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.enterprise.context.RequestScoped;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.GET;

@RequestScoped
@Authenticated
public class UserResource {

    @Inject
    UserMapper userMapper;
    @Inject
    UsersService usersService;

    ApelleUser user = null;

    public UserResource of(ApelleUser user) {
        this.user = user;
        return this;
    }

    @GET
    @Operation(summary = "The user data", description = "Returns the data of the user")
    @APIResponse(responseCode = "200", description = "The current user", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = UserQueryDto.class))
    })
    public UserQueryDto get() {
        return userMapper.toDto(user);
    }

    @DELETE
    @PermissionsAllowed("user-delete")
    @Transactional
    @Operation(summary = "Delete user", description = "Delete the user")
    @APIResponse(responseCode = "200", description = "The current user was deleted")
    public void delete() {
        user.delete();
    }

    @PermissionChecker("user-delete")
    boolean checkCanDelete(SecurityIdentity securityIdentity) {
        return user.getRoles().contains(ApelleUserRole.ADMIN) || isCurrent(securityIdentity);
    }

    private boolean isCurrent(SecurityIdentity securityIdentity) {
        return user.getId() == usersService.getCurrent().getId();
    }
}
