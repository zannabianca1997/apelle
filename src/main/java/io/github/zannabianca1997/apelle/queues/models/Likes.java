package io.github.zannabianca1997.apelle.queues.models;

import java.time.Instant;
import java.util.UUID;

import org.hibernate.annotations.Check;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.JoinColumn;
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
@EqualsAndHashCode(callSuper = false, of = { "user", "song", "givenAt" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "likes")
@Check(constraints = "count > 0")
@NamedNativeQuery(name = "Likes.countUserLikes", query = "SELECT COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queued_song_ref = :queued_song_ref AND l.user_id = :user_id), 0)", resultClass = Short.class)
/// A number of likes given on a song
public class Likes extends PanacheEntityBase {
    /// The user of the queue
    @NonNull
    @ManyToOne
    @Id
    @ToString.Exclude
    private ApelleUser user;

    /// The queued song
    @NonNull
    @Id
    @ToString.Exclude
    @ManyToOne
    @JoinColumn(name = "queued_song_ref", referencedColumnName = "ref")
    @OnDelete(action = OnDeleteAction.CASCADE)
    private QueuedSong song;

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
    public Likes(final @NonNull ApelleUser user, final @NonNull QueuedSong song, final Instant givenAt,
            final short count) {
        this.user = user;
        this.song = song;
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
    public static Likes findById(final ApelleUser user, final QueuedSong song, final Instant givenAt) {
        final var id = new Likes();
        id.user = user;
        id.song = song;
        id.givenAt = givenAt;
        return findById(id);
    }

    public static Likes findOldests(final QueueUser user) {
        return find("user = ?1 AND song.queue = ?2 ORDER BY givenAt ASC", user.getUser(), user.getQueue())
                .firstResult();
    }

    public static short givenBy(final UUID userId, final QueuedSong song) {
        return getSession()
                .createNamedQuery("Likes.countUserLikes", Short.class)
                .setParameter("queued_song_ref", song.getRef())
                .setParameter("user_id", userId)
                .getSingleResult();
    }

    public static long deleteReferringTo(final QueuedSong song) {
        return delete("song = ?1", song);
    }
}
