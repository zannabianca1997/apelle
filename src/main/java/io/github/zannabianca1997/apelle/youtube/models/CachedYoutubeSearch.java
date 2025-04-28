package io.github.zannabianca1997.apelle.youtube.models;

import java.util.ArrayList;

import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NonNull;

/**
 * Represent a search on youtube
 */
@Data
@AllArgsConstructor
public class CachedYoutubeSearch {
    @NonNull
    private ArrayList<SearchedSongQueryDto> found;
    private int totalResults;
    private String nextYoutubePageToken;
}
