package io.github.zannabianca1997.apelle.user.exceptions;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import lombok.Getter;

@Getter
public class UserNotFoundByIdException extends UserNotFoundException {
    private UUID userId;

    public UserNotFoundByIdException(UUID userId) {
        super(String.format("User `%s` not found", userId));
        this.userId = userId;
    }

    @APIResponse(responseCode = "404", description = "No user with this id", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<UserNotFoundByIdException> {
        @Override
        public Response toResponse(UserNotFoundByIdException exception) {
            return RestResponse.status(Status.NOT_FOUND, exception.getMessage()).toResponse();
        }
    }
}