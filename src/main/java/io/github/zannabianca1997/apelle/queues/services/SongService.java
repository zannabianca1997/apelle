package io.github.zannabianca1997.apelle.queues.services;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.dtos.sources.youtube.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.models.sources.youtube.YoutubeSong;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.VideoNotFoundException;
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
     * @throws BadYoutubeApiResponseException An error happened while talking to
     *                                        youtube
     * @throws VideoNotFoundException
     */
    private YoutubeSong fromDto(YoutubeSongAddDto youtubeSongAddDto)
            throws BadYoutubeApiResponseException, VideoNotFoundException {
        var videoData = youtubeService
                .getVideoData(youtubeSongAddDto.getVideoId());
        return songMapper.fromDto(youtubeSongAddDto, videoData);
    }

    /**
     * Complete the definition of a song by querying eventual apis
     * 
     * @param songAddDto The song to add
     * @return The completed song
     * @throws BadYoutubeApiResponseException An error happened while talking to
     *                                        youtube
     * @throws VideoNotFoundException
     */
    public Song fromDto(SongAddDto songAddDto) throws BadYoutubeApiResponseException, VideoNotFoundException {
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
