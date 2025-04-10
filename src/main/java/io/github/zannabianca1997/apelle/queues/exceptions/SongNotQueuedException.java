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
public class SongNotQueuedException extends Exception {
    private final UUID queueId;
    private final UUID songId;

    public SongNotQueuedException(UUID queueId, UUID songId) {
        super(String.format("Song `%s` is not queued inside queue %s", songId, queueId));
        this.queueId = queueId;
        this.songId = songId;
    }

    @Provider
    @APIResponse(responseCode = "404", description = "The song is not in the queue", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<SongAlreadyQueuedException> {
        @Override
        public Response toResponse(SongAlreadyQueuedException exception) {
            return RestResponse.status(Status.CONFLICT, exception.getMessage()).toResponse();
        }
    }
}
