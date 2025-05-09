package io.github.zannabianca1997.apelle.queues.roles.models;

import java.util.UUID;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;
import lombok.ToString;

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

    @Column(name = "max_likes", nullable = false)
    private short maxLikes;

    @Embedded
    @NonNull
    private QueueUserRolePermissions permissions;

}