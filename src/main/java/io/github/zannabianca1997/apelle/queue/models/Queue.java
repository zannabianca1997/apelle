package io.github.zannabianca1997.apelle.queue.models;

import java.sql.Timestamp;
import java.time.Duration;
import java.util.List;
import java.util.UUID;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.OneToMany;
import jakarta.persistence.OrderBy;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@AllArgsConstructor
@NoArgsConstructor
@Builder
@Entity
/// A queue of songs
public class Queue extends PanacheEntityBase {
    @NonNull
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /// Unique ID of the queue
    private UUID id;

    @Embeddable
    @Data
    @NoArgsConstructor
    @AllArgsConstructor
    @Builder
    public static class CurrentSong {
        @NonNull
        @ManyToOne
        @JoinColumn(name = "current_song")
        /// The song that is being played
        private Song song;

        /// Point in time where the song would need to be started to match with the
        /// execution.
        ///
        /// This might not correspond to an actual starting time. For example,
        /// if the user jumps forward or backward, this value changes.
        @NonNull
        @Column(name = "current_song_starts_at")
        private Timestamp startsAt;

        /**
         * Get the current position in the song
         * 
         * @return The duration from the start of the song
         */
        public Duration getPosition() {
            return Duration.ofMillis(System.currentTimeMillis() - this.startsAt.getTime());
        }
    }

    @Embedded
    /// The current playing song, if any
    private CurrentSong current;

    @OneToMany(cascade = CascadeType.ALL, mappedBy = "link.queue")
    @OrderBy("likes DESC, queued_at DESC")
    /// The songs in the queue
    private List<QueuedSong> queuedSongs;
}
