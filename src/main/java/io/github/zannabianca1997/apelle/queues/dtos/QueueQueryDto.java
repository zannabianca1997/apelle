package io.github.zannabianca1997.apelle.queues.dtos;

import java.util.List;
import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
@Schema(description = "A queue of songs")
public class QueueQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique ID of the queue")
    private UUID id;

    @Schema(description = "The current playing song, if any")
    private CurrentSongQueryDto current;

    @NonNull
    @JsonProperty(value = "queue", required = true)
    @Schema(description = "The songs in the queue")
    private List<QueuedSongShortQueryDto> queuedSongs;
}
