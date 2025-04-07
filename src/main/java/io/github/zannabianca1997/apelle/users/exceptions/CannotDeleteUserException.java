package io.github.zannabianca1997.apelle.users.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;

public class CannotDeleteUserException extends Exception {

    public CannotDeleteUserException() {
        super(String.format("Cannot delete user if not logged in as him, or with the role ADMIN"));
    }

    @Provider
    @APIResponse(responseCode = "403", description = "Cannot delete user if not logged in as him, or with the role `ADMIN`", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<CannotDeleteUserException> {
        @Override
        public Response toResponse(CannotDeleteUserException exception) {
            return RestResponse.status(Status.FORBIDDEN, exception.getMessage()).toResponse();
        }
    }
}
