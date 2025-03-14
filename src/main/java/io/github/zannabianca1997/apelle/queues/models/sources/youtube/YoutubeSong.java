package io.github.zannabianca1997.apelle.queues.models.sources.youtube;

import java.net.URI;
import java.net.URISyntaxException;
import java.time.Duration;

import org.apache.http.client.utils.URIBuilder;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

import io.github.zannabianca1997.apelle.queues.dtos.SongKind;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.youtube.ConstantUris;

@Data
@EqualsAndHashCode(callSuper = true)
@Entity
@Table(name = "youtube_song")
@NoArgsConstructor(access = AccessLevel.PROTECTED)
/**
 * A song backed up by a youtube video
 */
public class YoutubeSong extends Song {

    @NonNull
    @Column(name = "video_id", nullable = false)
    /// Code of the song
    private String videoId;

    @Override
    public URI getUri() {
        try {
            return new URIBuilder(ConstantUris.WATCH)
                    .addParameter("v", videoId)
                    .build();
        } catch (URISyntaxException e) {
            throw new RuntimeException("The youtube uri should always form a valid uri", e);
        }
    }

    @Override
    public SongKind getKind() {
        return SongKind.Youtube;
    }

    @Builder
    public YoutubeSong(
            @NonNull String name,
            @NonNull Duration duration,
            @NonNull String videoId) {
        super(name, duration);
        this.videoId = videoId;
    }
}
