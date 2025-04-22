package io.github.zannabianca1997.apelle.youtube.clients;

import org.eclipse.microprofile.rest.client.inject.RegisterRestClient;

import io.quarkus.rest.client.reactive.ClientQueryParam;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.QueryParam;
import lombok.NonNull;

import io.github.zannabianca1997.apelle.youtube.dtos.YoutubePaginatedDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;

@RegisterRestClient(configKey = "youtube-api")
@ClientQueryParam(name = "key", value = "${apelle.songs.sources.youtube.api-key}")
public interface YoutubeApiClient {
    @GET
    @Path("/videos")
    @ClientQueryParam(name = "part", value = "snippet,contentDetails")
    YoutubePaginatedDto<YoutubeVideoDataDto> getDataById(@NonNull @QueryParam("id") String videoId);

    @GET
    @Path("/search")
    @ClientQueryParam(name = "part", value = "snippet")
    @ClientQueryParam(name = "type", value = "video")
    @ClientQueryParam(name = "safeSearch", value = "none")
    @ClientQueryParam(name = "videoEmbeddable", value = "true")
    YoutubePaginatedDto<YoutubeVideoDataDto> getSearchByKeywords(
            @QueryParam("maxResults") int maxResults,
            @QueryParam("q") String query);

    @GET
    @Path("/search")
    @ClientQueryParam(name = "part", value = "snippet")
    @ClientQueryParam(name = "type", value = "video")
    @ClientQueryParam(name = "safeSearch", value = "none")
    @ClientQueryParam(name = "videoEmbeddable", value = "true")
    YoutubePaginatedDto<YoutubeVideoDataDto> getSearchPage(
            @QueryParam("maxResults") int maxResults,
            @QueryParam("q") String query,
            @QueryParam("pageToken") String pageToken);
}
