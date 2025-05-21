package io.github.zannabianca1997.apelle.queues.dtos;

import java.util.UUID;

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
@Schema(description = "Basic data about a song")
public class SongShortQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique id of the song")
    private UUID id;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Name of the song")
    private String name;
}
