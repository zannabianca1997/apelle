package io.github.zannabianca1997.apelle.queues.models;

import java.time.Instant;
import java.util.UUID;

import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.EmbeddedId;
import jakarta.persistence.Entity;
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
@Table(name = "likes")
/// A number of likes given on a song
public class Likes extends PanacheEntityBase {

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

        @NonNull
        @Column(nullable = false)
        /// The queued song
        private UUID song;

        @NonNull
        @Column(name = "given_at", nullable = false)
        /// When these likes where assigned
        private Instant givenAt;
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
    @ManyToOne
    @MapsId("song")
    /// The queued song
    private Song song;

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
    public Likes(
            ApelleUser user, QueuedSong song, Instant givenAt, short count) {
        this();
        this.link = new Link(null, null, null, givenAt);
        this.user = user;
        this.queue = song.getQueue();
        this.song = song.getSong();
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
    public static Likes findById(UUID user, QueuedSong.Link song, Instant givenAt) {
        return findById(new Link(user, song.getQueue(), song.getSong(), givenAt));
    }

    public static Likes findOldests(UUID user, QueuedSong.Link song) {
        return find("user_id = ?1 AND queue_id = ?2 AND song_id = ?3 ORDER BY given_at ASC",
                user, song.getQueue(), song.getSong())
                .firstResult();
    }

    public static long deleteReferringTo(QueuedSong.Link link) {
        return delete("queue_id = ?1 AND song_id = ?2", link.getQueue(), link.getSong());
    }
}
