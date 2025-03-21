package io.github.zannabianca1997.apelle.queues.models;

import java.util.UUID;

import org.hibernate.annotations.Formula;
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
import jakarta.persistence.NamedNativeQuery;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;

@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queue_user")
@NamedNativeQuery(name = "QueueUser.countLikes", query = "COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queue_id = :queue_id AND l.user_id = :user_id), 0)", resultClass = Short.class)
/// A user relationship with a queue
public class QueueUser extends PanacheEntityBase {

    @Embeddable
    @Data
    @NoArgsConstructor(access = AccessLevel.PROTECTED)
    @AllArgsConstructor(access = AccessLevel.PRIVATE)
    public static class Link {
        @NonNull
        @Column(nullable = false)
        /// The user
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

    @Formula("COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queue_id = queue_id AND l.user_id = user_id), 0)")
    @Setter(AccessLevel.NONE)
    /// Number of likes given by this user
    private short likes;

    @Builder
    public QueueUser(
            @NonNull ApelleUser user,
            @NonNull Queue queue,
            @NonNull QueueUserRole role,
            boolean likesFilled) {
        super();

        this.link = new Link();
        this.user = user;
        this.queue = queue;
        this.role = role;

        if (likesFilled) {
            this.likes = getSession()
                    .createNamedQuery("QueueUser.countLikes", Short.class)
                    .setParameter("queue_id", queue.getId())
                    .setParameter("user_id", user.getId())
                    .getSingleResult();
        } else {
            this.likes = 0;
        }
    }

    public short getMaxLikes() {
        return getRole().getMaxLikes();
    }

    public boolean canLike() {
        return getLikes() < getMaxLikes();
    }

    public static QueueUser findById(@NonNull UUID userId, @NonNull UUID queueId) {
        return findById(new Link(userId, queueId));
    }

}
