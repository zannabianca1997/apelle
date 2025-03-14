package io.github.zannabianca1997.apelle.youtube.dtos;

import java.net.URL;
import java.time.Duration;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.NonNull;

/**
 * Response of the youtube api
 */
@Data
public class VideoDataDto {
    @NonNull
    private String id;
    @NonNull
    private Snippet snippet;
    @NonNull
    private ContentDetails contentDetails;

    @Data
    public static class Snippet {
        @NonNull
        private String title;
        private Thumbnails thumbnails;

        @Data
        public static class Thumbnails {
            @JsonProperty("default")
            private Thumbnail default_;
            private Thumbnail medium;
            private Thumbnail high;
            private Thumbnail standard;
            private Thumbnail maxres;

            @Data
            public static class Thumbnail {
                private URL url;
                private int width;
                private int height;
            }
        }
    }

    @Data
    public static class ContentDetails {
        @NonNull
        private Duration duration;
    }
}
