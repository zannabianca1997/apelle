package io.github.zannabianca1997.apelle.youtube.dtos;

import java.util.List;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NonNull;

/**
 * Paginated response of the youtube api
 */
@Data
@AllArgsConstructor
public class YoutubePaginatedDto<T> {
    @NonNull
    private PageInfo pageInfo;
    @NonNull
    private List<T> items;

    @Data
    @AllArgsConstructor
    public static class PageInfo {
        private int totalResults;
    }

    public boolean isSinglePage() {
        return getPageInfo().getTotalResults() == getItems().size();
    }

    public boolean isSingleResult() {
        return isSinglePage() && getItems().size() == 1;
    }

    public T unwrapSingle() {
        if (!isSingleResult()) {
            return null;
        }
        return items.get(0);
    }

    public static <T> YoutubePaginatedDto<T> ofOne(T t) {
        return new YoutubePaginatedDto<T>(new PageInfo(1), List.of(t));
    }

    public static <T> YoutubePaginatedDto<T> ofNone() {
        return new YoutubePaginatedDto<T>(new PageInfo(0), List.of());
    }

}
