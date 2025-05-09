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
    QueueUserRoleQueuePermissionsQueryDto queue;

    @NonNull
    @JsonProperty(value = "queue_users", required = true)
    QueueUserRoleQueueUsersPermissionsQueryDto queueUsers;

    /** Can delete the queue */
    @JsonProperty(required = true)
    boolean delete;

}