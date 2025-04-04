package io.github.zannabianca1997.apelle.users.dtos;

import java.util.Set;
import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
@Jacksonized
@Schema(description = "Data about a single user")
public class UserQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique ID of the user")
    private UUID id;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique username of the user")
    private String name;

    @JsonProperty(required = true)
    @Schema(description = "Comma separated list of roles the user has")
    private Set<ApelleUserRole> roles;
}
