package io.github.zannabianca1997.apelle.queue.models;

import java.net.MalformedURLException;
import java.net.URISyntaxException;
import java.net.URL;
import java.sql.Timestamp;
import java.time.Duration;

import org.apache.http.client.utils.URIBuilder;

import io.github.zannabianca1997.apelle.queue.models.sources.youtube.YoutubeSong;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Embeddable
@Data
@NoArgsConstructor
@AllArgsConstructor
@Builder
public class PlayingSong {
    @NonNull
    @ManyToOne
    @JoinColumn(name = "playing_song")
    /// The song that is being played
    private Song song;

    /// Point in time where the song would need to be started to match with the
    /// execution.
    ///
    /// This might not correspond to an actual starting time. For example,
    /// if the user jumps forward or backward, this value changes.
    @NonNull
    @Column(name = "playing_song_starts_at")
    private Timestamp startsAt;

    /**
     * Get the current position in the song
     * 
     * @return The duration from the start of the song
     */
    public Duration getPosition() {
        return Duration.ofMillis(System.currentTimeMillis() - this.startsAt.getTime());
    }

    /**
     * Get a URL to the song, if available.
     * 
     * If possible the URL will contain the position info.
     * 
     * @return The URL, or null if not available.
     */
    public URL getUrl() {
        switch (song) {
            case YoutubeSong youtubeSong:
                try {
                    return new URIBuilder(youtubeSong.getUrl().toURI())
                            .addParameter("t", Long.toString(getPosition().toSeconds()))
                            .build().toURL();
                } catch (MalformedURLException | URISyntaxException e) {
                    throw new RuntimeException("The built url should be valid", e);
                }
            default:
                throw new RuntimeException("Missing url definition for class: `%s`".formatted(song.getClass()));
        }
    }
}