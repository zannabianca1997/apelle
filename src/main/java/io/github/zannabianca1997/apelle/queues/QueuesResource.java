package io.github.zannabianca1997.apelle.queues;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;

import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Response.Status;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.services.QueueService;

@Path("/queues")
@Tag(name = "Queue", description = "Management of the queue")
@Authenticated
public class QueuesResource {

    @Inject
    private QueueMapper queueMapper;
    @Inject
    private QueueService queueService;

    @POST
    @Transactional
    @Operation(summary = "Create a new queue", description = "Create a new queue without any song inside it")
    @APIResponse(responseCode = "201", description = "The newly created queue", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public RestResponse<QueueQueryDto> create() {
        var queue = queueService.create();
        return RestResponse.status(Status.CREATED, queueMapper.toDto(queue));
    }

}
