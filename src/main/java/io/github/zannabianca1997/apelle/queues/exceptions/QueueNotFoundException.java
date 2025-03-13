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
public class QueueNotFoundException extends Exception {
    private UUID queueId;

    public QueueNotFoundException(UUID queueId) {
        super(String.format("Queue `%s` not found", queueId));
        this.queueId = queueId;
    }

    @Provider
    @APIResponse(responseCode = "404", description = "No queue with this id", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<QueueNotFoundException> {
        @Override
        public Response toResponse(QueueNotFoundException exception) {
            return RestResponse.status(Status.NOT_FOUND, exception.getMessage()).toResponse();
        }
    }
}
