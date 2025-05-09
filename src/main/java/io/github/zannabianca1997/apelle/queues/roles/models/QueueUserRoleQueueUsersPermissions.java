package io.github.zannabianca1997.apelle.queues.roles.models;

import java.util.Set;

import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.JoinTable;
import jakarta.persistence.ManyToMany;
import lombok.Builder;
import lombok.Data;
import lombok.NonNull;

@Embeddable
@Data
@Builder
public class QueueUserRoleQueueUsersPermissions {
    @NonNull
    @ManyToMany
    @JoinTable(name = "permissions_queue_users_grant_roles", joinColumns = @JoinColumn(name = "role_id"), inverseJoinColumns = @JoinColumn(name = "granted_role_id"))
    private Set<QueueUserRole> grantRoles;

    @NonNull
    @ManyToMany
    @JoinTable(name = "permissions_queue_users_remove_roles", joinColumns = @JoinColumn(name = "role_id"), inverseJoinColumns = @JoinColumn(name = "removed_role_id"))
    private Set<QueueUserRole> removeRoles;

    @Column(name = "permissions_queue_users_ban", nullable = false)
    private boolean ban;
    @Column(name = "permissions_queue_users_remove", nullable = false)
    private boolean remove;
}