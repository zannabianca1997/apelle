package io.github.zannabianca1997.apelle.users.dtos;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
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
    private String roles;
}
