package io.github.zannabianca1997.apelle.queues.models;

import java.util.UUID;

import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.EmbeddedId;
import jakarta.persistence.Entity;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.MapsId;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
@Entity
@Table(name = "likes")
/// A number of likes given on a song
public class Likes extends PanacheEntityBase {

    @Embeddable
    @Data
    @NoArgsConstructor(access = AccessLevel.PROTECTED)
    @AllArgsConstructor(access = AccessLevel.PRIVATE)
    public static class Link {
        @NonNull
        @Column(nullable = false)
        /// The user
        private UUID user;

        @NonNull
        @Column(nullable = false)
        /// The queue
        private UUID queue;

        @NonNull
        @Column(nullable = false)
        /// The queued song
        private UUID song;
    }

    @EmbeddedId
    @NonNull
    private Link link;

    @NonNull
    @ManyToOne
    @MapsId("user")
    /// The user of the queue
    private ApelleUser user;

    @NonNull
    @ManyToOne
    @MapsId("queue")
    /// The queue
    private Queue queue;

    @NonNull
    @ManyToOne
    @MapsId("song")
    /// The queued song
    private Song song;

    /// Number of likes given
    private short count;
}
