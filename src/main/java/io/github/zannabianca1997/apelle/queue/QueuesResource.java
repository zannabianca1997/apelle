package io.github.zannabianca1997.apelle.queue;

import java.util.List;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;

import io.github.zannabianca1997.apelle.queue.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queue.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queue.models.Queue;
import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Response.Status;

@Path("/queues")
@Tag(name = "Queue", description = "Management of the queue")
@Authenticated
public class QueuesResource {

    @Inject
    QueueMapper queueMapper;

    @POST
    @Transactional
    @Operation(summary = "Create a new queue", description = "Create a new queue without any song inside it")
    @APIResponse(responseCode = "201", description = "The newly created queue", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public RestResponse<QueueQueryDto> create() {
        Queue queue = Queue.builder()
                .playing(null)
                .queuedSongs(List.of())
                .build();
        queue.persist();
        return RestResponse.status(Status.CREATED, queueMapper.toDto(queue));
    }

}
