package io.github.zannabianca1997.apelle.queues.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.Getter;

@Getter
public class RoleDoesNotExistException extends Exception {
    private final String name;

    public RoleDoesNotExistException(final String name) {
        super(String.format("Role `%s` does not exist", name));
        this.name = name;
    }

    @Provider
    @APIResponse(responseCode = "404", description = "No role with that name", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<RoleDoesNotExistException> {
        @Override
        public Response toResponse(final RoleDoesNotExistException exception) {
            return RestResponse.status(Status.NOT_FOUND, exception.getMessage()).toResponse();
        }
    }
}
