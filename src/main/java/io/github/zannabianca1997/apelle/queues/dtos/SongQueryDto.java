package io.github.zannabianca1997.apelle.queues.dtos;

import java.net.URL;
import java.time.Duration;
import java.util.Collection;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonProperty;

import io.github.zannabianca1997.apelle.common.dtos.SongKind;
import io.github.zannabianca1997.apelle.common.dtos.ThumbnailQueryDto;
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
public class SongQueryDto extends SongShortQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Duration of the song")
    @JsonFormat(shape = JsonFormat.Shape.STRING)
    private Duration duration;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Source of the song")
    private SongKind kind;

    @Schema(description = "Eventual public url of the song")
    private URL url;

    @Schema(description = "Available thumbnails for the song")
    private Collection<ThumbnailQueryDto> thumbnails;
}
