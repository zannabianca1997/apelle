package io.github.zannabianca1997.apelle.queues.models;

import java.util.List;
import java.util.UUID;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Embedded;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.OneToMany;
import jakarta.persistence.OrderBy;
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
/// A queue of songs
public class Queue extends PanacheEntityBase {
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /// Unique ID of the queue
    private UUID id;

    @Embedded
    /// The current playing song, if any
    private PlayingSong playing;

    @NonNull
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "link.queue")
    @OrderBy("likes DESC, queued_at ASC")
    /// The songs in the queue
    private List<QueuedSong> queuedSongs;
}
