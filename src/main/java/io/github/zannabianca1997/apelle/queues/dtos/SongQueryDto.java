package io.github.zannabianca1997.apelle.queues.dtos;

import java.net.URL;
import java.time.Duration;

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
/// Data about a song
public class SongQueryDto {
    @NonNull
    @JsonProperty(required = true)
    /// Name of the song
    private String name;

    @NonNull
    @JsonProperty(required = true)
    /// Duration of the song
    private Duration duration;

    @NonNull
    @JsonProperty(required = true)
    /// Kind of song
    ///
    /// This identifies the source (e.g. youtube, spotify, etc)
    private SongKind kind;

    /// Url of the song, if available
    private URL url;
}
