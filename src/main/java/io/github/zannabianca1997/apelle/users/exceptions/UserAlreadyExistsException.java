package io.github.zannabianca1997.apelle.users.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.Getter;

@Getter
public class UserAlreadyExistsException extends Exception {
    private String name;

    public UserAlreadyExistsException(String name) {
        super(String.format("A user named `%s` already exists", name));
        this.name = name;
    }

    @Provider
    @APIResponse(responseCode = "409", description = "The user already exist", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<UserAlreadyExistsException> {
        @Override
        public Response toResponse(UserAlreadyExistsException exception) {
            return RestResponse.status(Status.CONFLICT, exception.getMessage()).toResponse();
        }
    }
}
