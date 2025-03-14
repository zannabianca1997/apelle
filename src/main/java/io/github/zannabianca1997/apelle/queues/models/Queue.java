package io.github.zannabianca1997.apelle.queues.models;

import java.time.Instant;
import java.util.ArrayList;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.UUID;

import org.hibernate.annotations.Check;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.OneToMany;
import jakarta.persistence.OrderBy;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Singular;

@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Check(name = "song_is_either_started_or_stopped", constraints = """
        ((current_song IS NULL) AND (current_song_starts_at IS NULL) AND (current_song_position IS NULL))
        OR ((current_song IS NOT NULL) AND ((current_song_starts_at IS NULL) <> (current_song_position IS NULL)))
        """)
/// A queue of songs
public class Queue extends PanacheEntityBase {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /// Unique ID of the queue
    private UUID id;

    @Embedded
    /// The current playing song, if any
    private CurrentSong current;

    @NonNull
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "link.queue")
    @OrderBy("likes DESC, queued_at ASC")
    /// The songs in the queue
    private List<QueuedSong> queuedSongs;

    static public Queue empty() {
        return new Queue(null, new ArrayList<>());
    }

    /**
     * Order of the queued songs
     */
    private static final Comparator<QueuedSong> QUEUED_SONGS_COMPARATOR = Comparator
            .comparing(QueuedSong::getLikes).reversed()
            .thenComparing(QueuedSong::getQueuedAt);

    @Builder
    public Queue(CurrentSong current, @Singular @NonNull List<QueuedSong> queuedSongs) {
        super();
        // Sort the songs
        queuedSongs.sort(QUEUED_SONGS_COMPARATOR);

        this.id = null;
        this.current = current;
        this.queuedSongs = queuedSongs;
    }

    /**
     * Add a new song to the queue
     * 
     * @param song The song to add
     * @return The added song
     */
    public QueuedSong enqueue(@NonNull Song song) {
        var enqueued = QueuedSong.builder()
                .song(song)
                .queue(this)
                .likes((short) 0)
                .queuedAt(Instant.now())
                .build();
        // Add the song in the correct position
        int index = Collections.binarySearch(this.queuedSongs, enqueued, QUEUED_SONGS_COMPARATOR);
        if (index < 0) {
            index = -index - 1;
        }
        this.queuedSongs.add(index, enqueued);

        return enqueued;
    }
}
