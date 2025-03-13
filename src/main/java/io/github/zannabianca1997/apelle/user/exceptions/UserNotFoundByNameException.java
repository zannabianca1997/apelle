package io.github.zannabianca1997.apelle.user.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import lombok.Getter;

@Getter
public class UserNotFoundByNameException extends UserNotFoundException {
    private String username;

    public UserNotFoundByNameException(String username) {
        super(String.format("User named `%s` not found", username));
        this.username = username;
    }

    @APIResponse(responseCode = "404", description = "No user with this name", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<UserNotFoundByNameException> {
        @Override
        public Response toResponse(UserNotFoundByNameException exception) {
            return RestResponse.status(Status.NOT_FOUND, exception.getMessage()).toResponse();
        }
    }

}