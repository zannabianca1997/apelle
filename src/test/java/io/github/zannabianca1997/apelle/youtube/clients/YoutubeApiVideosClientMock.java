package io.github.zannabianca1997.apelle.youtube.clients;

import java.time.Duration;
import java.util.Arrays;
import java.util.Map;
import java.util.stream.Collectors;

import org.eclipse.microprofile.rest.client.inject.RestClient;

import io.github.zannabianca1997.apelle.youtube.dtos.PaginatedDto;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto.ContentDetails;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto.Snippet;
import io.quarkus.test.junit.QuarkusMock;
import jakarta.enterprise.context.ApplicationScoped;

/**
 * Mocks the youtube api.
 * 
 * This avoid calling the youtube api and enable running the tests even without
 * an api key
 */
@ApplicationScoped
public class YoutubeApiVideosClientMock implements YoutubeApiVideosClient {

    private static final record MockedVideo(
            String id, String title, Duration duration) {
    }

    private static final MockedVideo[] VIDEOS = {
            new MockedVideo("HvWFyINTNhj", "A test video", Duration.ofMinutes(5)),
            new MockedVideo("uDQDE6dsA5j", "Another test video", Duration.ofMinutes(2).plusSeconds(30)),
            new MockedVideo("y33obNyDHY5", "Oh, a test video", Duration.ofMinutes(29).plusSeconds(21))
    };

    public static final Map<String, PaginatedDto<VideoDataDto>> RESPONSES = Arrays.stream(VIDEOS).collect(
            Collectors.toMap(
                    MockedVideo::id,
                    mockedVideo -> PaginatedDto.ofOne(
                            VideoDataDto.builder()
                                    .id(mockedVideo.id())
                                    .snippet(Snippet.builder()
                                            .title(mockedVideo.title())
                                            .thumbnails(null)
                                            .build())
                                    .contentDetails(ContentDetails.builder()
                                            .duration(mockedVideo.duration())
                                            .build())
                                    .build())));

    public static final PaginatedDto<VideoDataDto> NOT_FOUND = PaginatedDto.ofNone();

    @Override
    public PaginatedDto<VideoDataDto> getDataById(String videoId) {
        return RESPONSES.getOrDefault(videoId, NOT_FOUND);
    }

    public static void install() {
        QuarkusMock.installMockForType(
                new YoutubeApiVideosClientMock(),
                YoutubeApiVideosClient.class,
                RestClient.LITERAL);
    }
}
