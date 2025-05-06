package io.github.zannabianca1997.apelle.youtube.dtos;

import java.time.Duration;

import lombok.Builder;
import lombok.Data;
import lombok.NonNull;
import lombok.extern.jackson.Jacksonized;

/**
 * Response of the youtube api
 */
@Data
@Builder
@Jacksonized
public class YoutubeVideoDataDto {
    @NonNull
    private String id;
    @NonNull
    private Snippet snippet;
    @NonNull
    private ContentDetails contentDetails;

    @Data
    @Builder
    @Jacksonized
    public static class Snippet {
        @NonNull
        private String title;
        private YoutubeThumbnailsDto thumbnails;

    }

    @Data
    @Builder
    @Jacksonized
    public static class ContentDetails {
        @NonNull
        private Duration duration;
    }
}
