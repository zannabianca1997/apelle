package io.github.zannabianca1997.apelle.youtube.models;

import java.net.URI;
import java.net.URISyntaxException;
import java.time.Duration;
import java.util.Collection;
import java.util.EnumMap;
import java.util.Map;

import org.apache.http.client.utils.URIBuilder;
import org.eclipse.microprofile.config.ConfigProvider;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.MapKey;
import jakarta.persistence.OneToMany;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import io.github.zannabianca1997.apelle.common.dtos.SongKind;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.models.Thumbnail;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = true, of = {})
@Entity
@Table(name = "youtube_song")
@OnDelete(action = OnDeleteAction.CASCADE)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
/**
 * A song backed up by a youtube video
 */
public class YoutubeSong extends Song {

    @NonNull
    @Column(name = "video_id", nullable = false, unique = true)
    /// Code of the song
    private String videoId;

    @OneToMany(cascade = CascadeType.ALL)
    @OnDelete(action = OnDeleteAction.CASCADE)
    @JoinColumn(name = "song_id")
    @MapKey(name = "size")
    @NonNull
    private Map<YoutubeThumbnailSize, YoutubeThumbnail> thumbnails;

    @Override
    public URI getUri() {
        try {
            return new URIBuilder(
                    ConfigProvider.getConfig()
                            .getValue("apelle.songs.sources.youtube.watch-uri", URI.class))
                    .addParameter("v", videoId)
                    .build();
        } catch (final URISyntaxException e) {
            throw new RuntimeException("The youtube uri should always form a valid uri", e);
        }
    }

    @Override
    public URI getUri(final @NonNull Duration position) {
        try {
            return new URIBuilder(getUri())
                    .addParameter("t", Long.toString(position.toSeconds()))
                    .build();
        } catch (final URISyntaxException e) {
            throw new RuntimeException("The built url should be valid", e);
        }
    }

    @Override
    public SongKind getKind() {
        return SongKind.Youtube;
    }

    @Override
    public Collection<Thumbnail> getAllThumbnails() {
        return getThumbnails().values().stream().map(t -> (Thumbnail) t).toList();
    }

    @Builder
    public YoutubeSong(
            final @NonNull String name,
            final @NonNull Duration duration,
            final @NonNull String videoId,
            final Map<YoutubeThumbnailSize, YoutubeThumbnail> thumbnails) {
        super(name, duration);
        this.videoId = videoId;
        this.thumbnails = thumbnails != null
                ? new EnumMap<>(thumbnails)
                : new EnumMap<>(YoutubeThumbnailSize.class);
    }

    @Override
    public boolean isSame(final Song b) {
        switch (b) {
            case final YoutubeSong bYoutubeSong:
                return getVideoId().equals(bYoutubeSong.getVideoId());
            default:
                return false;
        }
    }

    public static YoutubeSong findByVideoId(final @NonNull String videoId) {
        return YoutubeSong.<YoutubeSong>find("videoId", videoId).singleResultOptional().orElse(null);
    }
}
