package io.github.zannabianca1997.apelle.queues.roles.dtos;

import java.util.Set;
import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.NonNull;
import lombok.extern.jackson.Jacksonized;

@Data
@Builder
@Jacksonized
public class QueueUserRoleQueueUsersPermissionsQueryDto {
    @NonNull
    @JsonProperty(value = "grant_roles", required = true)
    private Set<UUID> grantRoles;

    @NonNull
    @JsonProperty(value = "remove_roles", required = true)
    private Set<UUID> removeRoles;

    @JsonProperty(required = true)
    private boolean ban;
    @JsonProperty(required = true)
    private boolean remove;
}