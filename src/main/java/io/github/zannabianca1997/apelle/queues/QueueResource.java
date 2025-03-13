package io.github.zannabianca1997.apelle.queues;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

@Path("/queues/{queueId}")
@Tag(name = "Queue", description = "Management of the queue")
@Authenticated
public class QueueResource {

    @Inject
    QueueMapper queueMapper;

    @GET
    @Operation(summary = "Get the queue state", description = "Get the queue state, with both the currently playing song and the list of songs to play next")
    @APIResponse(responseCode = "200", description = "The queue state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public QueueQueryDto get(UUID queueId) throws QueueNotFoundException {
        Queue queue = Queue.findById(queueId);
        if (queue == null) {
            throw new QueueNotFoundException(queueId);
        }
        return queueMapper.toDto(queue);
    }
}
