package io.github.zannabianca1997.apelle.queues.roles.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.parameters.Parameter;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;

import io.github.zannabianca1997.apelle.queues.roles.dtos.QueueUserRoleQueryDto;
import io.github.zannabianca1997.apelle.queues.roles.mappers.QueueUserRoleMapper;
import io.github.zannabianca1997.apelle.queues.roles.models.QueueUserRole;
import io.quarkus.security.Authenticated;
import io.smallrye.common.constraint.NotNull;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.NotFoundException;
import jakarta.ws.rs.Path;

@Path("/queues/roles")
@Tag(name = "Queue roles", description = "Roles of the user in a queue")
@Authenticated
public class QueueUserRoleResource {
    private final QueueUserRoleMapper queueUserRoleMapper;

    public QueueUserRoleResource(final QueueUserRoleMapper queueUserRoleMapper) {
        this.queueUserRoleMapper = queueUserRoleMapper;
    }

    @GET
    @Path("{id}")
    @Operation(summary = "Get a queue role", description = "Obtain details about a queue role, with permissions and maximum number of likes")
    public QueueUserRoleQueryDto get(
            final @NotNull @Parameter(description = "Queue role id") UUID id) {
        final QueueUserRole found = QueueUserRole.findById(id);
        if (found == null) {
            // TODO: make this a program exception
            throw new NotFoundException("The queue role %s does not exist".formatted(id));
        }
        return queueUserRoleMapper.toDto(found);
    }
}
