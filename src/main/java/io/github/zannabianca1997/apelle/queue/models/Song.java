package io.github.zannabianca1997.apelle.queue.models;

import java.time.Duration;
import java.util.UUID;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Inheritance;
import jakarta.persistence.InheritanceType;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@AllArgsConstructor
@NoArgsConstructor
@Entity
@Inheritance(strategy = InheritanceType.JOINED)
/// A song inserted in a queue
public abstract class Song extends PanacheEntityBase {
    @NonNull
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
}
