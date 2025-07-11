package io.github.zannabianca1997.apelle.common.dtos;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.smallrye.common.constraint.NotNull;
import jakarta.validation.constraints.PositiveOrZero;
import lombok.Data;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@SuperBuilder
@Jacksonized
@Schema(description = "Info about a page of responses")
public class PageInfo {
    @Schema(description = "Number of items in the search, if available", nullable = true, required = false)
    @JsonProperty(value = "total_items")
    @PositiveOrZero
    @NonNull
    private Integer totalItems;
    @Schema(description = "Number of items in the page", required = true)
    @NotNull
    @PositiveOrZero
    private int items;
    @Schema(description = "Page number", required = true)
    @NotNull
    @PositiveOrZero
    private int number;
    @Schema(description = "Next page token", required = false, nullable = true)
    private String next;
    @Schema(description = "Previous page token", required = false, nullable = true)
    private String prev;
}
