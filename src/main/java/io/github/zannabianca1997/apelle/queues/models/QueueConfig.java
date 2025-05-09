package io.github.zannabianca1997.apelle.queues.models;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.roles.models.QueueUserRole;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.Setter;
import lombok.ToString;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "id" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queue_config")
public class QueueConfig extends PanacheEntityBase {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /** Unique ID of the config */
    private UUID id;

    /**
     * If true, the user will like every song they add to the queue
     */
    @Column(nullable = false)
    private boolean autolike;

    /**
     * The role that will be given to users when they join the queue
     * <p>
     * TODO: When private queues will be implemented, this will be nullable marking
     * the queue as private (users will not be able to join if not added, and the
     * role is assigned to the one adding them)
     */
    @ManyToOne
    @JoinColumn(nullable = false, name = "default_role_id")
    private QueueUserRole defaultRole;

    /**
     * The role that will be given to the user that created the queue
     */
    @ManyToOne
    @JoinColumn(nullable = false, name = "creator_role_id")
    private QueueUserRole creatorRole;

    /**
     * The role that will be given to users when they are banned from the queue
     */
    @ManyToOne
    @JoinColumn(nullable = false, name = "banned_role_id")
    private QueueUserRole bannedRole;
}
