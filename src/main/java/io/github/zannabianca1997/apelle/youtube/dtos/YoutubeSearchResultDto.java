package io.github.zannabianca1997.apelle.youtube.dtos;

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
public class YoutubeSearchResultDto {
    @NonNull
    private Id id;
    @NonNull
    private Snippet snippet;

    @Data
    @Builder
    @Jacksonized
    public static class Id {
        @NonNull
        private String videoId;
    }

    @Data
    @Builder
    @Jacksonized
    public static class Snippet {
        @NonNull
        private String title;
        private YoutubeThumbnailsDto thumbnails;
    }

}
