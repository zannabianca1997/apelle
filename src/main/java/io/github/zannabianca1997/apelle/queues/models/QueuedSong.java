package io.github.zannabianca1997.apelle.queues.models;

import java.sql.Timestamp;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import jakarta.persistence.EmbeddedId;
import jakarta.persistence.Entity;
import jakarta.persistence.JoinColumn;
import jakarta.persistence.ManyToOne;
import jakarta.persistence.OneToOne;
import jakarta.persistence.Table;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@AllArgsConstructor
@NoArgsConstructor
@Builder
@Entity
@Table(name = "queued_song")
/// A queued song
public class QueuedSong extends PanacheEntityBase {

    @Embeddable
    @Data
    @NoArgsConstructor
    @AllArgsConstructor
    @Builder
    public static class Link {
        @NonNull
        @OneToOne(cascade = CascadeType.ALL)
        @JoinColumn(nullable = false)
        /// The queued song
        private Song song;

        @NonNull
        @ManyToOne
        @JoinColumn(nullable = false)
        /// The queue
        private Queue queue;
    }

    @EmbeddedId
    @NonNull
    private Link link;

    private short likes;

    @Column(name = "queued_at", nullable = false)
    private Timestamp queuedAt;
}
