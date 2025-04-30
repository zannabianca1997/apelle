package io.github.zannabianca1997.apelle.search.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.parameters.Parameter;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestQuery;

import io.github.zannabianca1997.apelle.common.dtos.Page;
import io.github.zannabianca1997.apelle.common.dtos.PageRequest;
import io.github.zannabianca1997.apelle.search.dtos.SearchedSongQueryDto;
import io.github.zannabianca1997.apelle.search.services.SearchService;
import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.validation.constraints.NotBlank;
import jakarta.ws.rs.BeanParam;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;

@Path("/search")
@Tag(name = "Search", description = "Search songs to add to queues")
@Authenticated
public class SearchResource {

    @Inject
    SearchService searchService;

    @GET
    @Operation(summary = "Search a song", description = """
            Search all available sources for a given song.

            The returned values are sorted by relevance. Each one contains the DTO one should send to the `/enqueue`
            endpoint to add the corresponding song.""")

    public Page<SearchedSongQueryDto> get(
            @RestQuery("q") @Parameter(description = "Searched song query") @NotBlank String query,
            @BeanParam PageRequest pageRequest) {
        return searchService.search(query, pageRequest);
    }
}
