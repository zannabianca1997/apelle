package io.github.zannabianca1997.apelle.queues.services;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.dtos.sources.youtube.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.models.sources.youtube.YoutubeSong;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponse;
import io.github.zannabianca1997.apelle.youtube.services.YoutubeService;

@ApplicationScoped
public class SongService {

    @Inject
    YoutubeService youtubeService;
    @Inject
    SongMapper songMapper;

    /**
     * Complete the definition of a song by querying the youtube API
     * 
     * @param youtubeSongAddDto The data provided by the user
     * @return The completed song entity
     * @throws BadYoutubeApiResponse An error happened while talking to youtube
     */
    private YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto) throws BadYoutubeApiResponse {
        var videoData = youtubeService
                .getVideoData(youtubeSongAddDto.getVideoId());
        return songMapper.fromDto(youtubeSongAddDto, videoData);
    }

    /**
     * Complete the definition of a song by querying eventual apis
     * 
     * @param songAddDto The song to add
     * @return The completed song
     * @throws BadYoutubeApiResponse An error happened while talking to youtube
     */
    public Song fromDto(SongAddDto songAddDto) throws BadYoutubeApiResponse {
        if (songAddDto == null) {
            return null;
        }
        switch (songAddDto) {
            case YoutubeSongAddDto youtubeSongAddDto:
                return fromDto(youtubeSongAddDto);
            default:
                throw new IllegalArgumentException(
                        String.format("Missing handler for class %s", songAddDto.getClass()));
        }
    }

}
