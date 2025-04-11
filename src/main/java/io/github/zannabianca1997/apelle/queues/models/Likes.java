package io.github.zannabianca1997.apelle.queues.models;

import java.time.Instant;
import java.util.UUID;

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
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "user", "queue", "song", "givenAt" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "likes")
@NamedNativeQuery(name = "Likes.countUserLikes", query = "SELECT COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queue_id = :queue_id AND l.song_id = :song_id AND l.user_id = :user_id), 0)", resultClass = Short.class)
/// A number of likes given on a song
public class Likes extends PanacheEntityBase {
    /// The user of the queue
    @NonNull
    @ManyToOne
    @Id
    private ApelleUser user;

    /// The queue
    @NonNull
    @ManyToOne
    @Id
    private Queue queue;

    /// The queued song
    @NonNull
    @ManyToOne
    @Id
    private Song song;

    /// When these likes where assigned
    @NonNull
    @Column(name = "given_at", nullable = false)
    @Id
    private Instant givenAt;

    /// Number of likes given
    private short count;

    /**
     * Create a new Likes.
     * 
     * @param user    The user liking
     * @param song    The song liked
     * @param givenAt The moment of the liking
     * @param count   The count of the likes
     */
    @Builder
    public Likes(@NonNull ApelleUser user, @NonNull QueuedSong song, Instant givenAt, short count) {
        this.user = user;
        this.queue = song.getQueue();
        this.song = song.getSong();
        this.givenAt = givenAt != null ? givenAt : Instant.now();
        this.count = count;
    }

    /**
     * Find the likes given by a user to a queued song, at a given instant
     * 
     * @param user    The user liking the song
     * @param song    The song liked
     * @param givenAt The moment the likes were given
     * @return The likes, or null if none were found
     */
    public static Likes findById(ApelleUser user, QueuedSong song, Instant givenAt) {
        var id = new Likes();
        id.user = user;
        id.queue = song.getQueue();
        id.song = song.getSong();
        id.givenAt = givenAt;
        return findById(id);
    }

    public static Likes findOldests(QueueUser user) {
        return find("user = ?1 AND queue = ?2 AND count > 0 ORDER BY link.givenAt ASC", user.getUser(), user.getQueue())
                .firstResult();
    }

    public static short givenBy(UUID user, QueuedSong song) {
        return getSession()
                .createNamedQuery("Likes.countUserLikes", Short.class)
                .setParameter("queue_id", song.getQueue().getId())
                .setParameter("song_id", song.getSong().getId())
                .setParameter("user_id", user)
                .getSingleResult();
    }

    public static long deleteReferringTo(QueuedSong song) {
        return delete("queue = ?1 AND song = ?2", song.getQueue(), song.getSong());
    }
}
