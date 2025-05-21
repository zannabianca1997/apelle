package io.github.zannabianca1997.apelle.queues.services;

import jakarta.enterprise.context.ApplicationScoped;

import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.YoutubeVideoNotFoundException;
import io.github.zannabianca1997.apelle.youtube.services.YoutubeService;

@ApplicationScoped
public class SongService {

    private final YoutubeService youtubeService;

    public SongService(final YoutubeService youtubeService) {
        this.youtubeService = youtubeService;
    }

    /**
     * Complete the definition of a song by querying eventual apis
     * 
     * @param songAddDto The song to add
     * @return The completed song
     * @throws BadYoutubeApiResponseException An error happened while talking to
     *                                        youtube
     * @throws YoutubeVideoNotFoundException
     */
    public Song fromDto(final SongAddDto songAddDto)
            throws BadYoutubeApiResponseException, YoutubeVideoNotFoundException {
        if (songAddDto == null) {
            return null;
        }
        switch (songAddDto) {
            case final YoutubeSongAddDto youtubeSongAddDto:
                return youtubeService.fromDto(youtubeSongAddDto);
            default:
                throw new IllegalArgumentException(
                        String.format("Missing handler for class %s", songAddDto.getClass()));
        }
    }

}
