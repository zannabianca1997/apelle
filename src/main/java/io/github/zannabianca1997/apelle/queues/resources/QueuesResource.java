package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.ResponseStatus;
import org.jboss.resteasy.reactive.RestResponse.StatusCode;

import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.services.QueueService;

@Path("/queues")
@Tag(name = "Queue", description = "Management of the queue")
@Authenticated
public class QueuesResource {

    @Inject
    QueueMapper queueMapper;
    @Inject
    QueueService queueService;
    @Inject
    QueueResource queueResource;

    @POST
    @Transactional
    @Operation(summary = "Create a new queue", description = "Create a new queue without any song inside it")
    @APIResponse(responseCode = "201", description = "The newly created queue", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    @ResponseStatus(StatusCode.CREATED)
    public QueueQueryDto create() {
        var queue = queueService.create();
        return queueMapper.toDto(queue, user -> 0);
    }

    @Path("/i/{queueId}")
    public QueueResource byId(UUID queueId) throws QueueNotFoundException {
        Queue queue = queueService.get(queueId);
        return queueResource.of(queue);
    }

    @Path("/c/{queueCode}")
    public QueueResource byCode(String queueCode) throws QueueNotFoundException {
        Queue queue = queueService.get(queueCode);
        return queueResource.of(queue);
    }
}
