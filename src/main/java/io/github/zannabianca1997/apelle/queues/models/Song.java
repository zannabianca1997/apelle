package io.github.zannabianca1997.apelle.queues.models;

import java.net.URI;
import java.time.Duration;
import java.util.UUID;

import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Inheritance;
import jakarta.persistence.InheritanceType;
import lombok.AccessLevel;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

import io.github.zannabianca1997.apelle.queues.dtos.SongKind;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;

@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Inheritance(strategy = InheritanceType.JOINED)
/// A song inserted in a queue
public abstract class Song extends PanacheEntityBase {
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /// Unique ID of the song
    private UUID id;

    @NonNull
    @Column(nullable = false)
    /// Name of the song
    private String name;

    @NonNull
    @Column(nullable = false)
    /// Duration of the song
    private Duration duration;

    /**
     * 
     * @return The song kind
     */
    public abstract SongKind getKind();

    /**
     * The uri for this song, if available
     * 
     * @return The uri, or null if not available
     */
    public URI getUri() {
        return null;
    }

    protected Song(
            @NonNull String name,
            @NonNull Duration duration) {
        super();
        this.id = null;
        this.name = name;
        this.duration = duration;
    }
}
