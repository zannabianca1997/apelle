package io.github.zannabianca1997.apelle.youtube.services;

import org.eclipse.microprofile.rest.client.inject.RestClient;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

import io.github.zannabianca1997.apelle.youtube.clients.YoutubeApiVideosClient;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponse;

@ApplicationScoped
public class YoutubeService {

    @Inject
    @RestClient
    YoutubeApiVideosClient youtubeApiVideosClient;

    public VideoDataDto getVideoData(String videoId) throws BadYoutubeApiResponse {
        var videos = youtubeApiVideosClient.getDataById(videoId);
        if (videos.getItems().size() > 1) {
            throw new BadYoutubeApiResponse("Multiple videos returned for a single id");
        }
        return videos.unwrapSingle();
    }
}
