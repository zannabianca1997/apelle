package io.github.zannabianca1997.apelle.youtube.services;

import org.eclipse.microprofile.rest.client.inject.RestClient;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import io.github.zannabianca1997.apelle.youtube.clients.YoutubeApiVideosClient;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.YoutubeVideoNotFoundException;
import io.github.zannabianca1997.apelle.youtube.mappers.YoutubeSongMapper;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeSong;

@ApplicationScoped
public class YoutubeService {

    @Inject
    @RestClient
    YoutubeApiVideosClient youtubeApiVideosClient;

    @Inject
    YoutubeSongMapper songMapper;

    private YoutubeVideoDataDto getVideoData(String videoId)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        var videos = youtubeApiVideosClient.getDataById(videoId);
        if (videos.getItems().size() > 1) {
            throw new BadYoutubeApiResponseException("Multiple videos returned for a single id");
        }
        if (videos.getItems().isEmpty()) {
            throw new YoutubeVideoNotFoundException(videoId);
        }
        return videos.unwrapSingle();
    }

    /**
     * Complete the definition of a song by querying the youtube API
     * 
     * @param youtubeSongAddDto The data provided by the user
     * @return The completed song entity
     * @throws BadYoutubeApiResponseException An error happened while talking to
     *                                        youtube
     * @throws YoutubeVideoNotFoundException
     */
    public YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        var videoData = getVideoData(youtubeSongAddDto.getVideoId());
        return songMapper.fromDto(youtubeSongAddDto, videoData);
    }
}
