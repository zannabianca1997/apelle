package io.github.zannabianca1997.apelle.queues;

import static io.restassured.RestAssured.given;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNull;
import java.net.MalformedURLException;
import java.util.UUID;

import org.jboss.resteasy.reactive.RestResponse.StatusCode;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;
import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;
import jakarta.transaction.Transactional;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongKind;
import io.github.zannabianca1997.apelle.queues.dtos.sources.youtube.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.sources.youtube.YoutubeSong;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.youtube.clients.YoutubeApiVideosClientMock;
import io.github.zannabianca1997.apelle.youtube.dtos.VideoDataDto;

@QuarkusTest
@TestHTTPEndpoint(QueueResource.class)
@Tag("queue")
class QueueResourceTest {
    @BeforeEach
    @Transactional
    void createUsers() {
        ApelleUser.deleteAll();
        ApelleUser.builder()
                .name("zanna")
                .password("zanna")
                .roles("user")
                .build().persist();
        ApelleUser.builder()
                .name("other")
                .password("other_psw")
                .roles("user")
                .build().persist();
    }

    UUID queueId;
    Queue createdQueue;

    @BeforeEach
    @Transactional
    void createQueue() {
        Queue.deleteAll();

        var queue = Queue.builder().build();
        queue.persist();

        this.queueId = queue.getId();
        this.createdQueue = queue;
    }

    @BeforeEach
    void installYoutubeMock() {
        YoutubeApiVideosClientMock.install();
    }

    @Test
    void shouldGetQueue() {
        QueueQueryDto created = given()
                .auth().basic("zanna", "zanna")
                .get("/", queueId).then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .extract().as(QueueQueryDto.class);

        assertNull(created.getCurrent());
        assertEquals(0, created.getQueuedSongs().size());
    }

    @Test
    void shouldEnqueueYoutubeSong() throws MalformedURLException {
        String videoId = YoutubeApiVideosClientMock.RESPONSES.keySet().iterator().next();
        VideoDataDto videoData = YoutubeApiVideosClientMock.RESPONSES.get(videoId).unwrapSingle();

        QueuedSongQueryDto created = given()
                .auth().basic("zanna", "zanna")
                .contentType(ContentType.JSON)
                .body(YoutubeSongAddDto.builder().videoId(videoId).build())
                .post("/queued-songs", queueId).then()
                .statusCode(StatusCode.CREATED)
                .contentType(ContentType.JSON)
                .extract().as(QueuedSongQueryDto.class);

        assertEquals(SongKind.Youtube, created.getKind());
        assertEquals(0, created.getLikes());
        assertEquals(videoData.getSnippet().getTitle(), created.getName());
        assertEquals(videoData.getContentDetails().getDuration(), created.getDuration());

        Queue queue = Queue.findById(queueId);

        assertEquals(1, queue.getQueuedSongs().size());

        QueuedSong enqueued = queue.getQueuedSongs().get(0);

        assertEquals(0, enqueued.getLikes());
        assertInstanceOf(YoutubeSong.class, enqueued.getSong());

        YoutubeSong song = (YoutubeSong) enqueued.getSong();

        assertEquals(videoId, song.getVideoId());
        assertEquals(created.getName(), song.getName());
        assertEquals(created.getDuration(), song.getDuration());
        assertEquals(created.getUrl(), song.getUri().toURL());
    }
}
