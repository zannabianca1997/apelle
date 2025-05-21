package io.github.zannabianca1997.apelle.queues.dtos;

import java.time.Instant;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
@Jacksonized
@Schema(description = "A song inside a queue")
public class QueuedSongShortQueryDto extends SongShortQueryDto {
    @NonNull
    @JsonProperty(value = "queued_at", required = true)
    @Schema(description = "The moment this song was added to the queue")
    private Instant queuedAt;
    @JsonProperty(required = true)
    @Schema(description = "The number of likes this song received")
    private short likes;
    @JsonProperty(required = true, value = "user_likes")
    @Schema(description = "The number of likes this song received by this user")
    private short userLikes;
}
