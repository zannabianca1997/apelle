package io.github.zannabianca1997.apelle.queues.dtos;

import java.net.URL;
import java.time.Duration;
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
@Schema(description = "A song")
public class SongQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique id of the song")
    private UUID id;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Name of the song")
    private String name;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Duration of the song")
    private Duration duration;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Source of the song")
    private SongKind kind;

    @Schema(description = "Eventual public url of the song")
    private URL url;
}
