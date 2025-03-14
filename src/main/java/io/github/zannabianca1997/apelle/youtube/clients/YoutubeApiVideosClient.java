package io.github.zannabianca1997.apelle.youtube.clients;

import org.eclipse.microprofile.rest.client.inject.RegisterRestClient;

import io.quarkus.rest.client.reactive.ClientQueryParam;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.QueryParam;
import lombok.NonNull;

import io.github.zannabianca1997.apelle.youtube.dtos.PaginatedDto;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto;

@Path("/videos")
@RegisterRestClient(configKey = "youtube-api")
@ClientQueryParam(name = "key", value = "${apelle.youtube.api.key}")
public interface YoutubeApiVideosClient {
    @GET
    @ClientQueryParam(name = "part", value = "snippet,contentDetails")
    PaginatedDto<VideoDataDto> getDataById(@NonNull @QueryParam("id") String videoId);
}
