package io.github.zannabianca1997.apelle.queues.models;

import java.util.UUID;

import org.hibernate.annotations.Formula;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;
import org.hibernate.annotations.Type;

import io.github.zannabianca1997.apelle.queues.configs.QueueUserRolesConfig.QueueUserRoleConfig.Permissions;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
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
@NamedNativeQuery(name = "QueueUser.countLikes", query = "SELECT COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queue_id = :queue_id AND l.user_id = :user_id), 0)", resultClass = Short.class)
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
    @Column(nullable = false)
    @Type(QueueUserRole.Type.class)
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

    public static QueueUser findById(@NonNull ApelleUser user, @NonNull Queue queue) {
        var id = new QueueUser();
        id.user = user;
        id.queue = queue;
        return findById(id);
    }

    public short getMaxLikes() {
        return getRole().getMaxLikes();
    }

    public Permissions getPermissions() {
        return getRole().getPermissions();
    }
}
