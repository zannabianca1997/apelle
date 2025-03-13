package io.github.zannabianca1997.apelle.queues.models.sources.youtube;

import java.net.MalformedURLException;
import java.net.URISyntaxException;
import java.net.URL;
import java.time.Duration;

import org.apache.http.client.utils.URIBuilder;

import io.github.zannabianca1997.apelle.queues.dtos.SongKind;
import io.github.zannabianca1997.apelle.queues.models.Song;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = true)
@Entity
@Table(name = "youtube_song")
@NoArgsConstructor(access = AccessLevel.PROTECTED)
public class YoutubeSong extends Song {

    @NonNull
    @Column(name = "video_id", nullable = false)
    /// Code of the song
    private String videoId;

    @Override
    public URL getUrl() {
        try {
            return new URIBuilder("https://www.youtube.com/watch")
                    .addParameter("v", videoId)
                    .build().toURL();
        } catch (MalformedURLException | URISyntaxException e) {
            throw new RuntimeException("The youtube url should always form a valid url", e);
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
