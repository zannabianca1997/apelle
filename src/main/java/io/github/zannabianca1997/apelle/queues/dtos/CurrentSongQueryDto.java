package io.github.zannabianca1997.apelle.queues.dtos;

import java.time.Duration;
import java.time.Instant;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
/// Data about a song being playied
public class CurrentSongQueryDto extends SongQueryDto {
    /// If the song is stopped or playing
    @JsonProperty(required = true)
    private boolean stopped;
    /// Point in time where the song would be started to reach this point
    @JsonProperty(required = true, value = "starts_at")
    private Instant startsAt;
    /// Position in the song
    @JsonProperty(required = true)
    private Duration position;
}
