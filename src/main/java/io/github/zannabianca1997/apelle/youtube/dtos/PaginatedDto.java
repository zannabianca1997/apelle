package io.github.zannabianca1997.apelle.youtube.dtos;

import java.util.List;

import lombok.Data;
import lombok.NonNull;

/**
 * Paginated response of the youtube api
 */
@Data
public class PaginatedDto<T> {
  @NonNull
  private PageInfo pageInfo;
  @NonNull
  private List<T> items;

  @Data
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
    return items.getFirst();
  }
}
