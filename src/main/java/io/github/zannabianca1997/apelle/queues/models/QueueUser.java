package io.github.zannabianca1997.apelle.queues.models;

import java.util.UUID;

import org.hibernate.annotations.JdbcType;
import org.hibernate.dialect.PostgreSQLEnumJdbcType;

import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.EmbeddedId;
import jakarta.persistence.Entity;
import jakarta.persistence.Enumerated;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.MapsId;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queue_user")
/// A user relationship with a queue
public class QueueUser extends PanacheEntityBase {

    @Embeddable
    @Data
    @NoArgsConstructor(access = AccessLevel.PROTECTED)
    @AllArgsConstructor(access = AccessLevel.PRIVATE)
    public static class Link {
        @NonNull
        @Column(nullable = false)
        /// The queued song
        private UUID user;

        @NonNull
        @Column(nullable = false)
        /// The queue
        private UUID queue;
    }

    @EmbeddedId
    @NonNull
    private Link link;

    @NonNull
    @ManyToOne
    @MapsId("user")
    /// The user of the queue
    private ApelleUser user;

    @NonNull
    @ManyToOne
    @MapsId("queue")
    /// The queue
    private Queue queue;

    @NonNull
    @Column(nullable = false)
    @Enumerated
    @JdbcType(PostgreSQLEnumJdbcType.class)
    /// Role of the user in the queue
    private QueueUserRole role;

    @Builder
    public QueueUser(
            @NonNull ApelleUser user,
            @NonNull Queue queue,
            @NonNull QueueUserRole role) {
        super();
        this.link = new Link();
        this.user = user;
        this.queue = queue;
        this.role = role;
    }

    public short getMaxLikes() {
        return getRole().getMaxLikes();
    }
}
