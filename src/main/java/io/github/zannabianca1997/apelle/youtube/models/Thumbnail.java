package io.github.zannabianca1997.apelle.youtube.models;

import java.util.UUID;

import jakarta.persistence.Column;
import jakarta.persistence.Embeddable;
import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.NonNull;

/**
 * A thumbnail returned from youtube
 */
public class Thumbnail {

    @Embeddable
    @Data
    @NoArgsConstructor(access = AccessLevel.PROTECTED)
    @AllArgsConstructor(access = AccessLevel.PRIVATE)
    public static class Link {
        @NonNull
        @Column(nullable = false)
        /// The queued song
        private UUID song;

        @NonNull
        @Column(nullable = false)
        /// The resolution name
        private String resolutionName;
    }
}
