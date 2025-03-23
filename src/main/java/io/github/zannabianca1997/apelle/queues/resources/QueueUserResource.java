package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import io.github.zannabianca1997.apelle.queues.dtos.QueueUserQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueUserMapper;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;

@Path("/queues/{queueId}/users")
@Tag(name = "Queue Users", description = "Management of the queue users")
@Authenticated
public class QueueUserResource {

    @Inject
    QueueUserService queueUserService;
    @Inject
    QueueUserMapper queueUserMapper;

    @GET
    @Path("/me")
    @Operation(summary = "Get the current queue user", description = "Get the state of the current queue user, with role and likes data.")
    @APIResponse(responseCode = "200", description = "The user state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueUserQueryDto.class))
    })
    public QueueUserQueryDto get(UUID queueId) throws QueueNotFoundException {
        QueueUser queueUser = queueUserService.getCurrent(queueId);
        return queueUserMapper.toDto(queueUser);
    }

    @GET
    @Path("/n/{userName}")
    @Operation(summary = "Get a queue user by name", description = "Get the state of a queue user by name, with role and likes data.")
    @APIResponse(responseCode = "200", description = "The user state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueUserQueryDto.class))
    })
    public QueueUserQueryDto get(UUID queueId, String userName)
            throws QueueNotFoundException, UserNotFoundByNameException {
        QueueUser queueUser = queueUserService.getByName(queueId, userName);
        return queueUserMapper.toDto(queueUser);
    }

    @GET
    @Path("/i/{userId}")
    @Operation(summary = "Get a queue user by id", description = "Get the state of a queue user by id, with role and likes data.")
    @APIResponse(responseCode = "200", description = "The user state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueUserQueryDto.class))
    })
    public QueueUserQueryDto get(UUID queueId, UUID userId) throws QueueNotFoundException, UserNotFoundByIdException {
        QueueUser queueUser = queueUserService.getById(queueId, userId);
        return queueUserMapper.toDto(queueUser);
    }
}
