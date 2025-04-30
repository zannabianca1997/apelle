package io.github.zannabianca1997.apelle.common.dtos;

import java.util.List;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Data;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@SuperBuilder
@Jacksonized
@Schema(description = "Paged response for Apelle API")
public class Page<T> {
    private List<T> items;
    @JsonProperty("page_info")
    private PageInfo pageInfo;
}