package io.github.zannabianca1997.apelle.youtube.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.experimental.StandardException;

@StandardException
public class BadYoutubeApiResponseException extends Exception {

    @Provider
    @APIResponse(responseCode = "502", description = "An unexpected response was given by the youtube api", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<BadYoutubeApiResponseException> {
        @Override
        public Response toResponse(final BadYoutubeApiResponseException exception) {
            return RestResponse.status(Status.BAD_GATEWAY, exception.getMessage()).toResponse();
        }
    }
}
