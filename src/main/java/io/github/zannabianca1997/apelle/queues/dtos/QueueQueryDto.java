package io.github.zannabianca1997.apelle.queues.dtos;

import java.util.List;
import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
/// A queue of songs
public class QueueQueryDto {
    @NonNull
    @JsonProperty(required = true)
    /// Unique ID of the queue
    private UUID id;

    /// The current playing song, if any
    private CurrentSongQueryDto current;

    @NonNull
    @JsonProperty(value = "queued_songs", required = true)
    /// The songs in the queue
    private List<QueuedSongQueryDto> queuedSongs;
}
