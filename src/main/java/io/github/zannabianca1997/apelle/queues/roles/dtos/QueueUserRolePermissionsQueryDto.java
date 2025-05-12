package io.github.zannabianca1997.apelle.queues.roles.dtos;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.NonNull;
import lombok.extern.jackson.Jacksonized;

@Data
@Builder
@Jacksonized
public class QueueUserRolePermissionsQueryDto {
    @NonNull
    @JsonProperty(required = true)
    private QueueUserRoleQueuePermissionsQueryDto queue;

    @NonNull
    @JsonProperty(value = "queue_users", required = true)
    private QueueUserRoleQueueUsersPermissionsQueryDto queueUsers;

    /** Can delete the queue */
    @JsonProperty(required = true)
    private boolean delete;
}