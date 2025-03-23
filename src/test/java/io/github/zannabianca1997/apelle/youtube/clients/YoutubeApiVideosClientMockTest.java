package io.github.zannabianca1997.apelle.youtube.clients;

import static org.junit.jupiter.api.Assertions.assertEquals;

import org.eclipse.microprofile.rest.client.inject.RestClient;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import io.quarkus.test.junit.QuarkusTest;
import jakarta.inject.Inject;

@QuarkusTest
@Tag("testEnviroment")
class YoutubeApiVideosClientMockTest {
    @Inject
    @RestClient
    YoutubeApiVideosClient youtubeApiVideosClient;

    @BeforeEach
    void installYoutubeMock() {
        YoutubeApiVideosClientMock.install();
    }

    @Test
    void isMocked() {
        String id = YoutubeApiVideosClientMock.RESPONSES.keySet().iterator().next();
        var gotten = youtubeApiVideosClient.getDataById(id);
        assertEquals(YoutubeApiVideosClientMock.RESPONSES.get(id), gotten);
    }
}
