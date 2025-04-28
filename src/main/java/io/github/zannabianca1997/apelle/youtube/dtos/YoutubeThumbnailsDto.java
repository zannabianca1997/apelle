package io.github.zannabianca1997.apelle.youtube.dtos;

import java.net.URL;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Data
@Builder
@Jacksonized
public class YoutubeThumbnailsDto {
    @JsonProperty("default")
    private Thumbnail default_;
    private Thumbnail medium;
    private Thumbnail high;
    private Thumbnail standard;
    private Thumbnail maxres;

    @Data
    @Builder
    @Jacksonized
    public static class Thumbnail {
        private URL url;
        private int width;
        private int height;
    }
}