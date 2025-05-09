package io.github.zannabianca1997.apelle.queues.roles.models;

import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.Embedded;
import lombok.Builder;
import lombok.Data;
import lombok.NonNull;

@Embeddable
@Data
@Builder
public class QueueUserRolePermissions {

    @NonNull
    @Embedded
    QueueUserRoleQueuePermissions queue;

    @NonNull
    @Embedded
    QueueUserRoleQueueUsersPermissions queueUsers;

    /** Can delete the queue */
    @Column(name = "permissions_delete", nullable = false)
    boolean delete;

}