package io.github.zannabianca1997.apelle.common.dtos;

import org.eclipse.microprofile.openapi.annotations.parameters.Parameter;
import org.jboss.resteasy.reactive.RestQuery;

import jakarta.validation.constraints.Max;
import jakarta.validation.constraints.Positive;
import jakarta.ws.rs.DefaultValue;
import lombok.Data;

@Data
public class PageRequest {
    @RestQuery
    @Parameter(description = "Page token. Must be obtained from a previous call to `/search`")
    private String page = null;

    @RestQuery("page_size")
    @DefaultValue("5")
    @Positive
    @Max(50)
    @Parameter(description = "Size of the requested page")
    private Integer pageSize = 5;
}
