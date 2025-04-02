package io.github.zannabianca1997.apelle.common.resources;

import org.eclipse.microprofile.config.inject.ConfigProperty;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;

import jakarta.annotation.security.PermitAll;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.Produces;
import jakarta.ws.rs.core.MediaType;

@Path("/version")
@Tag(name = "Other", description = "General endpoints")
public class VersionResource {

    @ConfigProperty(name = "quarkus.application.version")
    String version;

    @GET
    @Produces(MediaType.TEXT_PLAIN)
    @PermitAll
    @Operation(summary = "Version of the server", description = "Return the version of the server")
    @APIResponse(responseCode = "200", description = "The version of the server", content = {
            @Content(mediaType = "text/plain", example = "0.0.1", schema = @Schema(type = SchemaType.STRING, pattern = "\\d+\\.\\d+\\.\\d+"))
    })
    public String version() {
        return version;
    }
}
