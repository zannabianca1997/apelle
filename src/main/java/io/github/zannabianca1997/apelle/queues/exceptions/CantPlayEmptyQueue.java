package io.github.zannabianca1997.apelle.queues.exceptions;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.Getter;

@Getter
public class CantPlayEmptyQueue extends Exception {
    private final UUID queueId;

    public CantPlayEmptyQueue(UUID queueId) {
        super(String.format("Queue `%s` is empty, no song to play", queueId));
        this.queueId = queueId;
    }

    @Provider
    @APIResponse(responseCode = "400", description = "Cannot play an empty queue", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<CantPlayEmptyQueue> {
        @Override
        public Response toResponse(CantPlayEmptyQueue exception) {
            return RestResponse.status(Status.BAD_REQUEST, exception.getMessage()).toResponse();
        }
    }
}
