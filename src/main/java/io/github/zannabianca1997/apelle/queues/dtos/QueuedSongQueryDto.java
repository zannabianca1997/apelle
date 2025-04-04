package io.github.zannabianca1997.apelle.queues.dtos;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

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
}
