package io.github.zannabianca1997.apelle.queues.dtos;

import java.time.Duration;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
/// Data about a song being playied
public class PlayingSongQueryDto extends SongQueryDto {
    /// Position in the song
    @JsonProperty(required = true)
    private Duration position;
}
