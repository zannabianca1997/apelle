package io.github.zannabianca1997.apelle.youtube.models;

import java.util.ArrayList;

import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.quarkus.runtime.annotations.RegisterForReflection;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NonNull;

/**
 * Represent a search on youtube
 */
@Data
@AllArgsConstructor
// Needed as the class is only used for REDIS, and not automatically discovered
@RegisterForReflection
public class CachedYoutubeSearch {
    @NonNull
    private ArrayList<SearchedSongQueryDto> found;
    private int totalResults;
    private String nextYoutubePageToken;
}
