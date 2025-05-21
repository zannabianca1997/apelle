package io.github.zannabianca1997.apelle.search.dtos;

import java.net.URL;
import java.util.Collection;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.github.zannabianca1997.apelle.common.dtos.ThumbnailQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = false)
@SuperBuilder
@Jacksonized
@Schema(description = "A song searched from a probider")
public class SearchedSongQueryDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Name of the song")
    private String name;

    @NonNull
    @JsonProperty(value = "enqueue_data", required = true)
    @Schema(description = "Data to send to the `/enqueue` endpoint to add this song")
    private SongAddDto enqueueData;

    @Schema(description = "Eventual public url of the song")
    private URL url;

    @Schema(description = "Available thumbnails for the song")
    private Collection<ThumbnailQueryDto> thumbnails;
}
