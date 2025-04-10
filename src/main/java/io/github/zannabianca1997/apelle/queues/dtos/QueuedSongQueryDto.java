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
@Schema(description = "Full description of a song inside a queue")
public class QueuedSongQueryDto extends QueuedSongShortQueryDto {
    @JsonProperty(required = true, value = "user_likes")
    @Schema(description = "The number of likes this song received by this user")
    private short userLikes;
}
