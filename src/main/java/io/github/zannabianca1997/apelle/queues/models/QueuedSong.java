package io.github.zannabianca1997.apelle.queues.models;

import java.time.Instant;
import java.util.UUID;

import org.hibernate.annotations.Formula;
import org.hibernate.annotations.OnDelete;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.enterprise.context.BeforeDestroyed;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.PreRemove;
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
@EqualsAndHashCode(callSuper = false, of = { "queue", "song" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queued_song")
/// A queued song
public class QueuedSong extends PanacheEntityBase {

    /// The queued song
    @NonNull
    @ManyToOne(cascade = CascadeType.PERSIST)
    @Id
    private Song song;

    /// The queue
    @NonNull
    @ManyToOne
    @Id
    @ToString.Exclude
    private Queue queue;

    @NonNull
    @Column(name = "queued_at", nullable = false)
    private Instant queuedAt;

    @Formula("COALESCE((SELECT SUM(count) FROM Likes l WHERE l.queue_id = queue_id AND l.song_id = song_id), 0)")
    /// Number of likes on this song
    private short likes;

    @Builder
    public QueuedSong(
            @NonNull Song song,
            @NonNull Queue queue,
            @NonNull Instant queuedAt) {
        super();
        this.queuedAt = queuedAt;
        this.song = song;
        this.queue = queue;
    }

    public static QueuedSong findById(@NonNull UUID songId, @NonNull Queue queue) {
        return Song.<Song>findByIdOptional(songId).map(song -> findById(song, queue)).orElse(null);
    }

    public static QueuedSong findById(@NonNull Song song, @NonNull Queue queue) {
        var id = new QueuedSong();
        id.song = song;
        id.queue = queue;
        return findById(id);
    }

}
