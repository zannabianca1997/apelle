package io.github.zannabianca1997.apelle.queues.models;

import java.net.URI;
import java.net.URISyntaxException;
import java.time.Duration;
import java.time.Instant;

import org.apache.http.client.utils.URIBuilder;

import io.github.zannabianca1997.apelle.queues.models.sources.youtube.YoutubeSong;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import lombok.AccessLevel;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;

@Embeddable
@Data
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class CurrentSong {
    @NonNull
    @ManyToOne
    @JoinColumn(name = "current_song")
    /// The song that is being played
    private Song song;

    /// Point in time where the song would need to be started to match with the
    /// execution. Valid only for running songs.
    ///
    /// This might not correspond to an actual starting time. For example,
    /// if the user jumps forward or backward, this value changes.
    @Column(name = "current_song_starts_at")
    @Setter(AccessLevel.NONE)
    private Instant startsAt;

    public Instant getStartsAt() {
        if (startsAt != null) {
            return startsAt;
        }
        return Instant.now().minus(position);
    }

    /// Position in the song. Valid only for stopped songs
    @Column(name = "current_song_position")
    @Setter(AccessLevel.NONE)
    private Duration position;

    public Duration getPosition() {
        if (position != null) {
            return position;
        }
        return Duration.between(this.startsAt, Instant.now());
    }

    public boolean isStopped() {
        return position != null;
    }

    public void setStopped(boolean stopping) {
        if (stopping) {
            stop();
        } else {
            play();
        }
    }

    /**
     * Stop the song
     * 
     * @return if the song was playing before
     */
    public boolean stop() {
        if (isStopped()) {
            return false;
        }

        var stopPosition = getPosition();

        position = stopPosition;
        startsAt = null;

        return true;
    }

    /**
     * Start the song
     * 
     * @return if the song state changed
     */
    public boolean play() {
        if (!isStopped()) {
            return false;
        }

        var playStartsAt = getStartsAt();

        if (!playStartsAt.plus(getPosition()).isBefore(Instant.now())) {
            // The song reached his end, not starting it
            return false;
        }

        position = null;
        startsAt = playStartsAt;

        return true;
    }

    /**
     * Get a URI to the song, if available.
     * 
     * If possible the URI will contain the position info.
     * 
     * @return The URI, or null if not available.
     */
    public URI getUri() {
        switch (song) {
            case YoutubeSong youtubeSong:
                try {
                    return new URIBuilder(youtubeSong.getUri())
                            .addParameter("t", Long.toString(getPosition().toSeconds()))
                            .build();
                } catch (URISyntaxException e) {
                    throw new RuntimeException("The built url should be valid", e);
                }
            default:
                return song.getUri();
        }
    }

    public static CurrentSongBuilder builder() {
        return new CurrentSongBuilder();
    }

    public static class CurrentSongBuilder {
        public Song song;

        public CurrentSongBuilder song(@NonNull Song song) {
            this.song = song;
            return this;
        }

        public Stopped stopped() {
            return new Stopped().song(song);
        }

        public static class Stopped extends CurrentSongBuilder {
            @Override
            public Stopped song(Song song) {
                this.song = song;
                return this;
            }

            public Duration position;

            public Stopped position(@NonNull Duration position) {
                this.position = position;
                return this;
            }

            public CurrentSong build() {
                if (song == null) {
                    throw new NullPointerException("field song is marked non-null but is null");
                }
                if (position == null) {
                    position = Duration.ZERO;
                }

                var built = new CurrentSong();
                built.song = song;
                built.position = position;
                built.startsAt = null;
                return built;
            }
        }

        public Playing playing() {
            return new Playing().song(song);
        }

        public static class Playing extends CurrentSongBuilder {
            public Playing song(Song song) {
                this.song = song;
                return this;
            }

            public Instant startsAt;

            public Playing startsAt(@NonNull Instant startsAt) {
                this.startsAt = startsAt;
                return this;
            }

            public CurrentSong build() {
                if (song == null) {
                    throw new NullPointerException("field song is marked non-null but is null");
                }
                if (startsAt == null) {
                    startsAt = Instant.now();
                }

                var built = new CurrentSong();
                built.song = song;
                built.position = null;
                built.startsAt = startsAt;
                return built;
            }
        }
    }

    /**
     * Calculate how much time left before the song is finished
     * 
     * @return The time left
     */
    public Duration timeLeft() {
        return getSong().getDuration().minus(getPosition());
    }
}