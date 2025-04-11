package io.github.zannabianca1997.apelle.youtube.models;

import java.net.URL;
import java.util.UUID;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;
import org.hibernate.type.SqlTypes;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.EmbeddedId;
import jakarta.persistence.Entity;
import jakarta.persistence.Id;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.MapsId;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

/**
 * A thumbnail returned from youtube
 */
@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "youtube_thumbnail")
public class YoutubeThumbnail extends PanacheEntityBase {
    /// The song this thumbnail belongs to
    @NonNull
    @ManyToOne
    @OnDelete(action = OnDeleteAction.CASCADE)
    @Id
    private YoutubeSong song;

    @NonNull
    @Column(nullable = false)
    @JdbcTypeCode(SqlTypes.NAMED_ENUM)
    @Id
    /// The resolution
    private YoutubeThumbnailSize size;

    @Column(nullable = false)
    @NonNull
    private URL url;

    private int width;

    private int height;

    @Builder
    public YoutubeThumbnail(@NonNull YoutubeThumbnailSize size, @NonNull URL url, int width, int height) {
        this.size = size;
        this.url = url;
        this.width = width;
        this.height = height;
    }
}
