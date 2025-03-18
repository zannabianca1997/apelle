package io.github.zannabianca1997.apelle.queues.dtos;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
@Jacksonized
@Schema(description = "A song inside a queue")
public class QueuedSongQueryDto extends SongQueryDto {
    @JsonProperty(required = true)
    @Schema(description = "The number of likes this song received")
    private short likes;
}
