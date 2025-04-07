package io.github.zannabianca1997.apelle.configs.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;

import io.github.zannabianca1997.apelle.queues.configs.QueueUserRolesConfig;
import io.github.zannabianca1997.apelle.queues.exceptions.RoleDoesNotExistException;
import io.github.zannabianca1997.apelle.queues.models.QueueUserRole;
import io.github.zannabianca1997.apelle.queues.services.QueueUserRolesService;
import jakarta.annotation.security.PermitAll;
import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

@Path("configs/queue-user/roles")
@Tag(name = "Configs", description = "Configuration of the server")
@PermitAll
public class QueueUserRolesConfigsResource {

    @Inject
    QueueUserRolesConfig queueUserRolesConfig;
    @Inject
    QueueUserRolesService queueUserRolesService;

    @GET
    @Operation(summary = "Get the list of roles and the default ones", description = "The list of all roles and the default ones.")
    public QueueUserRolesConfig get() {
        return queueUserRolesConfig;
    }

    @GET
    @Path("{roleName}")
    @Operation(summary = "Get the configuration for a role", description = "The extended configuration for a role.")
    public QueueUserRole getRole(String roleName) throws RoleDoesNotExistException {
        return queueUserRolesService.getRole(roleName);
    }
}
