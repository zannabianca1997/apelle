package io.github.zannabianca1997.apelle.queues.dtos;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
/// Data about a queued song
public class QueuedSongQueryDto extends SongQueryDto {
    /// The number of likes this song received
    @JsonProperty(required = true)
    private short likes;
}
