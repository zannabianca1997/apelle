package io.github.zannabianca1997.apelle.users.exceptions;

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
public class UserNotFoundByIdException extends UserNotFoundException {
    private final UUID userId;

    public UserNotFoundByIdException(final UUID userId) {
        super(String.format("User `%s` not found", userId));
        this.userId = userId;
    }

    @Provider
    @APIResponse(responseCode = "404", description = "No user with this id", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<UserNotFoundByIdException> {
        @Override
        public Response toResponse(final UserNotFoundByIdException exception) {
            return RestResponse.status(Status.NOT_FOUND, exception.getMessage()).toResponse();
        }
    }
}