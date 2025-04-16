package io.github.zannabianca1997.apelle.queues.dtos;

import java.util.List;
import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
@Jacksonized
@Schema(description = "A queue of songs")
public class QueueQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique ID of the queue")
    private UUID id;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique code of the queue")
    private String code;

    @Schema(description = "The current playing song, if any")
    private CurrentSongQueryDto current;

    @NonNull
    @JsonProperty(value = "queue", required = true)
    @Schema(description = "The songs in the queue")
    private List<QueuedSongShortQueryDto> queuedSongs;

    @NonNull
    @JsonProperty(value = "player_state_id", required = true)
    @Schema(description = """
            Id of the current state of the player

            This is an opaque id that is regenerated at each modification of the playing
            song. Requests can be conditional on the state they refer to, so they are
            refused in case of a mismatch.""")
    private UUID playerStateId;
}
