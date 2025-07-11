package io.github.zannabianca1997.apelle.queues.models;

import java.net.URI;
import java.security.InvalidParameterException;
import java.time.Duration;
import java.time.Instant;

import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import lombok.AccessLevel;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;
import lombok.ToString;

/**
 * Data about a song currently playing
 * 
 * The API exposed by this class emulate an important property: the song
 * position is always between the two extremes. Even if never stopped, this
 * class will show as if it naturally stopped after it came to his natural end.
 * This also means that the state of the database needs not to be updated as
 * songs stops, as songs that are completed will naturally be considered
 * stopped.
 */
@Embeddable
@EqualsAndHashCode
@ToString
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class CurrentSong {
    @NonNull
    @ManyToOne
    @JoinColumn(name = "current_song")
    @Getter
    @Setter
    /// The song that is being played
    private Song song;

    /// Point in time where the song would need to be started to match with the
    /// execution. Valid only for running songs.
    ///
    /// This might not correspond to an actual starting time. For example,
    /// if the user jumps forward or backward, this value changes.
    @Column(name = "current_song_starts_at")
    @Setter(AccessLevel.PRIVATE)
    private Instant startsAt;

    public Instant getStartsAt() {
        final Instant now = Instant.now();

        if (startsAt == null) {
            return now.minus(position);
        }
        // Check: if it passed the end, it is considered stopped
        final Duration timePassed = Duration.between(startsAt, now);
        final Duration duration = getSong().getDuration();
        return timePassed.compareTo(duration) < 0 ? startsAt : now.minus(duration);
    }

    /// Position in the song. Valid only for stopped songs
    @Column(name = "current_song_position")
    @Setter(AccessLevel.PRIVATE)
    private Duration position;

    public Duration getPosition() {
        if (position != null) {
            return position;
        }
        // If a song passed the end, is considered stopped by default
        final Duration timePassed = Duration.between(startsAt, Instant.now());
        final Duration duration = getSong().getDuration();
        return timePassed.compareTo(duration) < 0 ? timePassed : duration;
    }

    // Stopped information

    public boolean isStopped() {
        return !(position == null && startsAt.plus(getSong().getDuration()).isAfter(Instant.now()));
    }

    // Action on the current song

    /**
     * Stop the song
     * 
     * @return if the song was playing before
     */
    public boolean stop() {
        if (isStopped()) {
            return false;
        }

        final var stopPosition = getPosition();

        setPosition(stopPosition);
        setStartsAt(null);

        return true;
    }

    /**
     * Start the song
     * 
     * @return if the song state changed
     */
    public boolean start() {
        if (!isStopped()) {
            return false;
        }

        final var playStartsAt = getStartsAt();

        if (playStartsAt.plus(getSong().getDuration()).isBefore(Instant.now())) {
            // The song reached his end, not starting it
            return false;
        }

        setPosition(null);
        setStartsAt(playStartsAt);

        return true;
    }

    /**
     * Jump to the given position
     * 
     * @param position The position
     */
    public void jumpTo(@NonNull Duration position) {
        // Clamp to the song duration
        if (position.isNegative()) {
            position = Duration.ZERO;
        }
        // Set the position or the starting time
        // This also mantains the stopped by time/stopped by player state
        if (isStopped()) {
            setPosition(position);
            setStartsAt(null);
        } else {
            setPosition(null);
            setStartsAt(Instant.now().minus(position));
        }
    }

    /**
     * Get a URI to the song, if available.
     * 
     * If possible the URI will contain the position info.
     * 
     * @return The URI, or null if not available.
     */
    public URI getUri() {
        return song.getUri(getPosition());
    }

    public static CurrentSongBuilder builder() {
        return new CurrentSongBuilder();
    }

    public static class CurrentSongBuilder {
        public Song song;

        public CurrentSongBuilder song(@NonNull final Song song) {
            this.song = song;
            return this;
        }

        public Stopped stopped() {
            return new Stopped().song(this.song);
        }

        public static class Stopped extends CurrentSongBuilder {
            @Override
            public Stopped song(final Song song) {
                this.song = song;
                return this;
            }

            public Duration position;

            public Stopped position(@NonNull final Duration position) {
                this.position = position;
                return this;
            }

            public CurrentSong build() {
                if (this.song == null) {
                    throw new NullPointerException("field song is marked non-null but is null");
                }
                if (this.position == null) {
                    this.position = Duration.ZERO;
                } else if (this.position.compareTo(this.song.getDuration()) < 0) {
                    throw new InvalidParameterException("Cannot make a song positioned after its end");
                }

                final var built = new CurrentSong();
                built.song = this.song;
                built.position = this.position;
                built.startsAt = null;
                return built;
            }
        }

        public Playing playing() {
            return new Playing().song(this.song);
        }

        public static class Playing extends CurrentSongBuilder {
            public Playing song(final Song song) {
                this.song = song;
                return this;
            }

            public Instant startsAt;

            public Playing startsAt(@NonNull final Instant startsAt) {
                this.startsAt = startsAt;
                return this;
            }

            public CurrentSong build() {
                if (this.song == null) {
                    throw new NullPointerException("field song is marked non-null but is null");
                }
                final Instant now = Instant.now();
                if (this.startsAt == null) {
                    this.startsAt = now;
                } else if (startsAt.isAfter(now)) {
                    throw new InvalidParameterException("Cannot make a song starting in the future");
                }

                final var built = new CurrentSong();
                built.song = this.song;
                built.position = null;
                built.startsAt = this.startsAt;
                return built;
            }
        }
    }

    /**
     * Calculate how much time left before the song is finished
     * 
     * @return The time left
     */
    public Duration getTimeLeft() {
        return getSong().getDuration().minus(getPosition());
    }
}