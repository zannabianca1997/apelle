package io.github.zannabianca1997.apelle.common.dtos;

import java.util.List;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@SuperBuilder
@Jacksonized
@Schema(description = "Paged response for Apelle API")
public class Page<T> {
    @NonNull
    @JsonProperty(required = true)
    private List<T> items;
    @NonNull
    @JsonProperty(value = "page_info", required = true)
    private PageInfo pageInfo;
}