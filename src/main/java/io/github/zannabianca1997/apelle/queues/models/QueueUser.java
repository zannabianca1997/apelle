package io.github.zannabianca1997.apelle.queues.models;

import org.hibernate.annotations.Formula;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import io.github.zannabianca1997.apelle.queues.roles.models.QueueUserRole;
import io.github.zannabianca1997.apelle.queues.roles.models.QueueUserRolePermissions;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.NamedNativeQuery;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;
import lombok.ToString;
import lombok.Getter;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "user", "queue" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queue_user")
@NamedNativeQuery(name = "QueueUser.countLikes", query = """
        SELECT COALESCE((
            SELECT SUM(count)
            FROM likes l
            JOIN queued_song s ON s.ref = l.queued_song_ref
            WHERE s.queue_id = :queue_id
              AND l.user_id = :user_id)
        , 0)""", resultClass = Short.class)
/// A user relationship with a queue
public class QueueUser extends PanacheEntityBase {

    /// The user of the queue
    @NonNull
    @ManyToOne
    @OnDelete(action = OnDeleteAction.CASCADE)
    @Id
    private ApelleUser user;

    /// The queue
    @NonNull
    @ManyToOne
    @OnDelete(action = OnDeleteAction.CASCADE)
    @Id
    @ToString.Exclude
    private Queue queue;

    @NonNull
    @ManyToOne
    @JoinColumn(nullable = false)
    /// Role of the user in the queue
    private QueueUserRole role;

    @Formula("""
            COALESCE((
                SELECT SUM(count)
                FROM likes l
                JOIN queued_song s ON s.ref = l.queued_song_ref
                WHERE s.queue_id = queue_id
                  AND l.user_id = user_id)
            , 0)""")
    @Setter(AccessLevel.NONE)
    /// Number of likes given by this user
    private short likes;

    @Builder
    public QueueUser(
            final @NonNull ApelleUser user,
            final @NonNull Queue queue,
            final @NonNull QueueUserRole role,
            final boolean likesFilled) {
        super();

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

    public static QueueUser findById(final @NonNull ApelleUser user, final @NonNull Queue queue) {
        final var id = new QueueUser();
        id.user = user;
        id.queue = queue;
        return findById(id);
    }

    public short getMaxLikes() {
        return getRole().getMaxLikes();
    }

    public short getAvailableLikes() {
        return (short) (getMaxLikes() - getLikes());
    }

    public QueueUserRolePermissions getPermissions() {
        return getRole().getPermissions();
    }
}
