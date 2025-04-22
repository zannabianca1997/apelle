package io.github.zannabianca1997.apelle.common.dtos;

import com.fasterxml.jackson.annotation.JsonValue;

import lombok.AccessLevel;
import lombok.AllArgsConstructor;
import lombok.Getter;

/**
 * Kind of songs that the server handles
 */
@AllArgsConstructor(access = AccessLevel.PRIVATE)
public enum SongKind {
    Youtube(Constants.YOUTUBE);

    @JsonValue
    @Getter
    private final String value;

    public static class Constants {
        public static final String YOUTUBE = "Youtube";
    }
}
