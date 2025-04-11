package io.github.zannabianca1997.apelle.queues.models;

import java.net.URI;
import java.time.Duration;
import java.util.Collection;
import java.util.UUID;

import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Inheritance;
import jakarta.persistence.InheritanceType;
import jakarta.persistence.OneToMany;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Setter;
import lombok.ToString;
import lombok.Getter;

import io.github.zannabianca1997.apelle.queues.dtos.SongKind;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "id" })
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Inheritance(strategy = InheritanceType.JOINED)
@Table(name = "song")
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

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.REMOVE, mappedBy = "song")
    /// The likes on this song, on any queue
    private Collection<Likes> likes;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.REMOVE, mappedBy = "song")
    @ToString.Exclude
    /// The queues this song is inside
    private Collection<QueuedSong> queues;

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

    /**
     * The uri for this song at the given time, if available
     * 
     * @return The uri, or null if not available
     */
    public URI getUri(Duration time) {
        return getUri();
    }

    protected Song(
            @NonNull String name,
            @NonNull Duration duration) {
        super();
        this.id = null;
        this.name = name;
        this.duration = duration;
    }

    /**
     * Check if two songs are the same song
     * 
     * @param b The other song
     * @return If the two song match
     */
    public boolean isSame(Song b) {
        return false;
    }
}
