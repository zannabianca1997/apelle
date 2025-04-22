package io.github.zannabianca1997.apelle.youtube.clients;

import java.time.Duration;
import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collectors;

import org.eclipse.microprofile.rest.client.inject.RestClient;

import io.github.zannabianca1997.apelle.youtube.dtos.YoutubePaginatedDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto.ContentDetails;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto.Snippet;
import io.quarkus.test.junit.QuarkusMock;
import jakarta.enterprise.context.ApplicationScoped;

/**
 * Mocks the youtube api.
 * 
 * This avoid calling the youtube api and enable running the tests even without
 * an api key
 */
@ApplicationScoped
public class YoutubeApiClientMock implements YoutubeApiClient {

    private static final record MockedVideo(
            String id, String title, Duration duration) {
    }

    private static final MockedVideo[] VIDEOS = {
            new MockedVideo("HvWFyINTNhj", "A test video", Duration.ofMinutes(5)),
            new MockedVideo("uDQDE6dsA5j", "Another test video", Duration.ofMinutes(2).plusSeconds(30)),
            new MockedVideo("y33obNyDHY5", "Oh, a test video", Duration.ofMinutes(29).plusSeconds(21))
    };

    public static final Map<String, YoutubePaginatedDto<YoutubeVideoDataDto>> RESPONSES = Arrays.stream(VIDEOS).collect(
            Collectors.toMap(
                    MockedVideo::id,
                    mockedVideo -> YoutubePaginatedDto.ofOne(
                            YoutubeVideoDataDto.builder()
                                    .id(mockedVideo.id())
                                    .snippet(Snippet.builder()
                                            .title(mockedVideo.title())
                                            .thumbnails(null)
                                            .build())
                                    .contentDetails(ContentDetails.builder()
                                            .duration(mockedVideo.duration())
                                            .build())
                                    .build())));

    public static final YoutubePaginatedDto<YoutubeVideoDataDto> NOT_FOUND = YoutubePaginatedDto.ofNone();

    @Override
    public YoutubePaginatedDto<YoutubeVideoDataDto> getDataById(String videoId) {
        return RESPONSES.getOrDefault(videoId, NOT_FOUND);
    }

    public static void install() {
        QuarkusMock.installMockForType(
                new YoutubeApiClientMock(),
                YoutubeApiClient.class,
                RestClient.LITERAL);
    }
}
