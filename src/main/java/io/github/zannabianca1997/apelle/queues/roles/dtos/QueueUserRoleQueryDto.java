package io.github.zannabianca1997.apelle.queues.roles.dtos;

import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.NonNull;
import lombok.extern.jackson.Jacksonized;

/**
 * Role that a user has in a queue
 */
@Data
@Builder
@Jacksonized
public class QueueUserRoleQueryDto {
    /** Unique ID of the config */
    @JsonProperty(required = true, access = JsonProperty.Access.READ_ONLY)
    @NonNull
    private UUID id;

    @JsonProperty(required = true)
    @NonNull
    private String name;

    @JsonProperty(value = "max_likes", required = true)
    private short maxLikes;

    @NonNull
    @JsonProperty(required = true)
    private QueueUserRolePermissionsQueryDto permissions;

}