package io.github.zannabianca1997.apelle.queues.models;

import java.util.Set;
import java.util.UUID;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;
import lombok.ToString;
import lombok.extern.jackson.Jacksonized;

/**
 * Role that a user has in a queue
 */
@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "id" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queue_user_role")
public class QueueUserRole extends PanacheEntityBase {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /** Unique ID of the config */
    private UUID id;

    @NonNull
    @Column(nullable = false)
    private String name;

    @Column(name = "max_likes")
    private short maxLikes;

    @Embedded
    @NonNull
    private Permissions permissions;

    @Embeddable
    @Data
    @Builder
    @Jacksonized
    public static class Permissions {

        @NonNull
        @Embedded
        QueuePermissions queue;

        @NonNull
        @Embedded
        QueueUsersPermissions queueUsers;

        /** Can delete the queue */
        @Column(name = "permissions_delete")
        boolean delete;

        @Embeddable
        @Data
        @Builder
        @Jacksonized
        public static class QueuePermissions {
            @Column(name = "permissions_queue_start")
            private boolean start;
            @Column(name = "permissions_queue_stop")
            private boolean stop;
            @Column(name = "permissions_queue_next")
            private boolean next;
            @Column(name = "permissions_queue_like")
            private boolean like;
            @Column(name = "permissions_queue_enqueue")
            private boolean enqueue;
            @Column(name = "permissions_queue_remove")
            private boolean remove;
            @Column(name = "permissions_queue_ban")
            private boolean ban;
        }

        @Embeddable
        @Data
        @Builder
        @Jacksonized
        public static class QueueUsersPermissions {
            @NonNull
            @Column(name = "permissions_queue_users_grant_roles", nullable = false)
            private Set<String> grantRoles;
            @NonNull
            @Column(name = "permissions_queue_users_remove_roles", nullable = false)
            private Set<String> removeRoles;

            @Column(name = "permissions_queue_users_ban")
            private boolean ban;
            @Column(name = "permissions_queue_users_remove")
            private boolean remove;
        }
    }
}
