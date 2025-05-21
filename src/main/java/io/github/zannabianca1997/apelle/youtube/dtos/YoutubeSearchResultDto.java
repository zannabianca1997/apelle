package io.github.zannabianca1997.apelle.youtube.dtos;

import java.util.Objects;

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
        // Will be null on things that are not video
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

    public boolean isVideo() {
        return Objects.nonNull(getId().getVideoId());
    }

}
