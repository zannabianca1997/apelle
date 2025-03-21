package io.github.zannabianca1997.apelle.queues.models;

import java.time.Instant;
import java.util.UUID;

import org.hibernate.annotations.Formula;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.EmbeddedId;
import jakarta.persistence.Entity;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.MapsId;
import jakarta.persistence.OneToOne;
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
@Table(name = "queued_song")
/// A queued song
public class QueuedSong extends PanacheEntityBase {

    @Embeddable
    @Data
    @NoArgsConstructor(access = AccessLevel.PROTECTED)
    @AllArgsConstructor(access = AccessLevel.PRIVATE)
    public static class Link {
        @NonNull
        @Column(nullable = false)
        /// The queued song
        private UUID song;

        @NonNull
        @Column(nullable = false)
        /// The queue
        private UUID queue;
    }

    @EmbeddedId
    @NonNull
    private Link link;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToOne(cascade = CascadeType.ALL)
    @MapsId("song")
    /// The queued song
    private Song song;

    @NonNull
    @ManyToOne
    @MapsId("queue")
    /// The queue
    private Queue queue;

    @NonNull
    @Column(name = "queued_at", nullable = false)
    private Instant queuedAt;

    @Formula("COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queue_id = queue_id AND l.song_id = song_id), 0)")
    @Setter(AccessLevel.NONE)
    /// Number of likes on this song
    private short likes;

    @Builder
    public QueuedSong(
            @NonNull Song song,
            @NonNull Queue queue,
            @NonNull Instant queuedAt) {
        super();
        this.link = new Link();
        this.queuedAt = queuedAt;
        this.song = song;
        this.queue = queue;
    }
}
