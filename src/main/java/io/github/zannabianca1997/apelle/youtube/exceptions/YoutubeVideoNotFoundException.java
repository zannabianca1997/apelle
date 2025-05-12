package io.github.zannabianca1997.apelle.youtube.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.Getter;
import lombok.NonNull;

@Getter
public class YoutubeVideoNotFoundException extends Exception {
    private final String videoId;

    public YoutubeVideoNotFoundException(final @NonNull String videoId) {
        super("The video `%s` does not exist".formatted(videoId));
        this.videoId = videoId;
    }

    @Provider
    @APIResponse(responseCode = "404", description = "The requested video does not exist", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<YoutubeVideoNotFoundException> {
        @Override
        public Response toResponse(final YoutubeVideoNotFoundException exception) {
            return RestResponse.status(Status.NOT_FOUND, exception.getMessage()).toResponse();
        }
    }
}
