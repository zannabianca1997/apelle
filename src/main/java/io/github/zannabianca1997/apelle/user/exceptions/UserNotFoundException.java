package io.github.zannabianca1997.apelle.user.exceptions;

import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.jboss.resteasy.reactive.RestResponse;

import jakarta.ws.rs.core.Response;
import jakarta.ws.rs.core.Response.Status;
import jakarta.ws.rs.ext.ExceptionMapper;
import lombok.Getter;

@Getter
public abstract class UserNotFoundException extends Exception {
    protected UserNotFoundException(String message) {
        super(message);
    }

}
