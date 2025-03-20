package io.github.zannabianca1997.apelle.queues.exceptions;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import io.github.zannabianca1997.apelle.queues.models.Song;
import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.Getter;

@Getter
public class SongAlreadyQueued extends Exception {
    private final UUID queueId;
    private final UUID songId;

    public SongAlreadyQueued(UUID queueId, Song song) {
        super(String.format("Song `%s` is already queued with id `%s", song.getName(), song.getId()));
        this.queueId = queueId;
        this.songId = song.getId();
    }

    @Provider
    @APIResponse(responseCode = "409", description = "The song is already in the queue", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<SongAlreadyQueued> {
        @Override
        public Response toResponse(SongAlreadyQueued exception) {
            return RestResponse.status(Status.CONFLICT, exception.getMessage()).toResponse();
        }
    }
}
