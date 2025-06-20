package io.github.zannabianca1997.apelle.queues.roles.models;

import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.Embedded;
import lombok.AccessLevel;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Embeddable
@Data
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class QueueUserRolePermissions {
    @NonNull
    @Embedded
    private QueueUserRoleQueuePermissions queue;

    @NonNull
    @Embedded
    private QueueUserRoleQueueUsersPermissions queueUsers;

    /** Can delete the queue */
    @Column(name = "permissions_delete", nullable = false)
    private boolean delete;
}