package io.github.zannabianca1997.apelle.queue.models.sources.youtube;

import io.github.zannabianca1997.apelle.queue.models.Song;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = true)
@Entity
@AllArgsConstructor
@NoArgsConstructor
public class YoutubeSong extends Song {

    @NonNull
    @Column(nullable = false)
    /// Code of the song
    private String code;

}
