package io.github.zannabianca1997.apelle.youtube.clients;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import io.quarkus.test.junit.QuarkusTest;

@QuarkusTest
@Tag("testEnviroment")
class YoutubeApiClientMockTest {
    private final YoutubeApiClient youtubeApiVideosClient;

    public YoutubeApiClientMockTest(final @RestClient YoutubeApiClient youtubeApiVideosClient) {
        this.youtubeApiVideosClient = youtubeApiVideosClient;
    }

    @BeforeEach
    void installYoutubeMock() {
        YoutubeApiClientMock.install();
    }

    @Test
    void isMocked() {
        final String id = YoutubeApiClientMock.RESPONSES.keySet().iterator().next();
        final var gotten = youtubeApiVideosClient.getDataById(id);
        assertEquals(YoutubeApiClientMock.RESPONSES.get(id), gotten);
    }
}
