package io.github.zannabianca1997.apelle.queues.dtos;

import java.time.Duration;
import java.time.Instant;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
@Schema(description = "The song currently being played")
public class CurrentSongQueryDto extends SongQueryDto {
    @JsonProperty(required = true)
    @Schema(description = "If the song is currently stopped")
    private boolean stopped;
    @JsonProperty(required = true, value = "starts_at")
    @Schema(description = "Moment at which the song should have started to reach the current position")
    private Instant startsAt;
    @JsonProperty(required = true)
    @JsonFormat(shape = JsonFormat.Shape.STRING)
    @Schema(description = "Current position in the song")
    private Duration position;
}
