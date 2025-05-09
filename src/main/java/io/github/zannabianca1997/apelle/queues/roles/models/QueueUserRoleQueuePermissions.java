package io.github.zannabianca1997.apelle.queues.roles.models;

import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import lombok.Builder;
import lombok.Data;

@Embeddable
@Data
@Builder
public class QueueUserRoleQueuePermissions {
    @Column(name = "permissions_queue_start", nullable = false)
    private boolean start;
    @Column(name = "permissions_queue_stop", nullable = false)
    private boolean stop;
    @Column(name = "permissions_queue_next", nullable = false)
    private boolean next;
    @Column(name = "permissions_queue_like", nullable = false)
    private boolean like;
    @Column(name = "permissions_queue_enqueue", nullable = false)
    private boolean enqueue;
    @Column(name = "permissions_queue_remove", nullable = false)
    private boolean remove;
    @Column(name = "permissions_queue_ban", nullable = false)
    private boolean ban;
}
