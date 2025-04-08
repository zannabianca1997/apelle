package io.github.zannabianca1997.apelle.queues.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import io.github.zannabianca1997.apelle.queues.models.QueueUserRole;
import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import jakarta.ws.rs.ext.Provider;
import lombok.Getter;
import lombok.NonNull;

@Getter
public class ActionNotPermitted extends Exception {
    private final String roleName;
    private final String action;

    public ActionNotPermitted(@NonNull QueueUserRole role, @NonNull String action) {
        super(String.format("Action `%s` is not permitted for role `%s`", action, role.getName()));
        this.roleName = role.getName();
        this.action = action;
    }

    @Provider
    @APIResponse(responseCode = "403", description = "The action is forbidden", content = {
            @Content(mediaType = "text/plain")
    })
    public static class Mapper implements ExceptionMapper<ActionNotPermitted> {
        @Override
        public Response toResponse(ActionNotPermitted exception) {
            return RestResponse.status(Status.FORBIDDEN, exception.getMessage()).toResponse();
        }
    }
}
