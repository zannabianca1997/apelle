package io.github.zannabianca1997.apelle.queues.models;

import java.time.Duration;
import java.time.Instant;
import java.util.ArrayList;
import java.util.Collection;
import java.util.Collections;
import java.util.Comparator;
import java.util.List;
import java.util.UUID;
import java.util.stream.Stream;

import org.hibernate.annotations.Check;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueueException;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.OneToMany;
import jakarta.persistence.OrderBy;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;
import lombok.Singular;
import lombok.ToString;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "id" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "queue")
@Check(name = "song_is_either_started_or_stopped", constraints = """
        -- Either the current song is started or it's stopped
        (
            -- The current song is null
            (current_song IS NULL)
            AND (current_song_starts_at IS NULL)
            AND (current_song_position IS NULL)
        ) OR (
            -- Only one of the time reference is filled in
            (current_song IS NOT NULL)
            AND (
                (current_song_starts_at IS NULL) <> (current_song_position IS NULL)
            )
        )
        """)
/// A queue of songs
public class Queue extends PanacheEntityBase {

    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /// Unique ID of the queue
    private UUID id;

    @NonNull
    @Column(nullable = false, unique = true)
    /// Unique remembrable queue code
    private String code;

    @Embedded
    /// The current playing song, if any
    private CurrentSong current;

    @NonNull
    @Column(name = "player_state_id", nullable = false)
    @Setter(AccessLevel.PRIVATE)
    /// Id of the current state of the player
    ///
    /// This is an opaque id that is regenerated at each modification of the playing
    /// song. Requests can be conditional on the state they refer to, so they are
    /// refused in case of a mismatch.
    private UUID playerStateId;

    private void setPlayerStateId() {
        setPlayerStateId(UUID.randomUUID());
    }

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "queue", orphanRemoval = true)
    @OrderBy("likes DESC, queued_at ASC")
    /// The songs in the queue
    private List<QueuedSong> queuedSongs;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "queue", orphanRemoval = true)
    /// The users of this queue
    private Collection<QueueUser> users;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "queue")
    /// The likes on this queue
    private Collection<Likes> likes;

    /**
     * Order of the queued songs
     */
    private static final Comparator<QueuedSong> QUEUED_SONGS_COMPARATOR = Comparator
            // First order by likes
            .comparing(QueuedSong::getLikes).reversed()
            // Then order by time of insertion
            .thenComparing(QueuedSong::getQueuedAt);

    @Builder
    public Queue(CurrentSong current, @Singular @NonNull List<QueuedSong> queuedSongs, @NonNull String code) {
        super();
        // Sort the songs
        queuedSongs.sort(QUEUED_SONGS_COMPARATOR);

        this.id = null;
        this.current = current;
        setPlayerStateId();
        this.queuedSongs = queuedSongs;
        this.users = new ArrayList<>();
        this.likes = new ArrayList<>();

        this.code = code;
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
                .queuedAt(Instant.now())
                .build();
        // Add the song in the correct position
        int index = Collections.binarySearch(this.queuedSongs, enqueued, QUEUED_SONGS_COMPARATOR);
        if (index >= 0) {
            throw new RuntimeException(
                    "Tried to add a song that is already in the queue. This need to be checked before");
        }
        index = -index - 1;

        List<QueuedSong> editQueuedSongs = getQueuedSongs();
        editQueuedSongs.add(index, enqueued);
        setQueuedSongs(editQueuedSongs);

        enqueued.persist();

        return enqueued;
    }

    /**
     * Start to play music
     * 
     * @return If the queue was stopped before
     * @throws CantPlayEmptyQueueException The queue is empty
     */
    public boolean start() throws CantPlayEmptyQueueException {
        // If a song is running, start playing
        if (getCurrent() != null) {
            var started = getCurrent().start();
            if (started) {
                setPlayerStateId();
            }
            return started;
        }

        // Pop a song from the queue
        if (getQueuedSongs().isEmpty()) {
            throw new CantPlayEmptyQueueException(getId());
        }

        QueuedSong next = getQueuedSongs().remove(0);
        Likes.deleteReferringTo(next);
        setCurrent(CurrentSong.builder()
                .song(next.getSong())
                .playing().startsAt(Instant.now())
                .build());

        setPlayerStateId();
        return true;
    }

    /**
     * Stop the queue from playing
     * 
     * @return If the queue was playing before
     */
    public boolean stop() {
        if (getCurrent() == null) {
            return false;
        }

        var stopped = getCurrent().stop();
        if (stopped) {
            setPlayerStateId();
        }
        return stopped;
    }

    /**
     * Move to the next song
     * 
     * @return If the queue was playing before
     * @throws CantPlayEmptyQueueException The queue is empty
     */
    public void next() throws CantPlayEmptyQueueException {
        if (getCurrent() != null) {
            if (getQueuedSongs().isEmpty()) {
                getCurrent().jumpTo(Duration.ZERO);
            } else {
                // Pop the current song and put it in the queue
                var removingCurrent = getCurrent().getSong();
                setCurrent(null);
                enqueue(removingCurrent);
            }
            setPlayerStateId();
        }
        start();
    }

    /**
     * Get all songs tracked by this queue
     * 
     * @return All the song tracked
     */
    public Stream<Song> getAllSongs() {
        return Stream.concat(
                Stream.ofNullable(getCurrent()).map(CurrentSong::getSong),
                getQueuedSongs().stream().map(QueuedSong::getSong));
    }

    public static boolean exists(UUID queueId) {
        return findById(queueId) != null;
    }

    public void sortSongs() {
        List<QueuedSong> sortingQueuedSongs = getQueuedSongs();
        sortingQueuedSongs.sort(QUEUED_SONGS_COMPARATOR);
        setQueuedSongs(sortingQueuedSongs);
    }

    public static Queue findByCode(String queueCode) {
        return Queue.<Queue>find("code", queueCode).singleResultOptional().orElse(null);
    }

    public static boolean existByCode(String queueCode) {
        return Queue.find("code", queueCode).count() > 0;
    }
}
