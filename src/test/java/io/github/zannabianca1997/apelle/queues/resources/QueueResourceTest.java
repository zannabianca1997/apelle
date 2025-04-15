package io.github.zannabianca1997.apelle.queues.resources;

import static io.restassured.RestAssured.given;
import static org.junit.jupiter.api.Assertions.assertAll;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.net.MalformedURLException;
import java.util.Arrays;
import java.util.UUID;
import org.jboss.resteasy.reactive.RestResponse.StatusCode;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import com.google.common.collect.Streams;

import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongShortQueryDto;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.services.QueueUserRolesService;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;
import io.github.zannabianca1997.apelle.youtube.clients.YoutubeApiVideosClientMock;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeSongAddDto;
import io.github.zannabianca1997.apelle.youtube.dtos.YoutubeVideoDataDto;
import io.github.zannabianca1997.apelle.youtube.models.YoutubeSong;

@QuarkusTest
@Tag("queue")
@TestHTTPEndpoint(QueuesResource.class)
class QueueResourceTest {

    UUID queueId;
    Queue createdQueue;

    @Inject
    QueueUserRolesService queueUserRolesService;

    @BeforeEach
    @Transactional
    void createUsersAndQueues() {
        ApelleUser.deleteAll();
        Queue.deleteAll();
        Song.deleteAll();

        ApelleUser admin = ApelleUser.builder()
                .name("zanna")
                .password("zanna")
                .role(ApelleUserRole.USER)
                .build();
        admin.persist();

        ApelleUser.builder()
                .name("other")
                .password("other_psw")
                .role(ApelleUserRole.USER)
                .build().persist();

        var queue = Queue.builder()
                .code("code")
                .build();
        queue.getUsers().add(QueueUser.builder()
                .queue(queue)
                .user(admin)
                .role(queueUserRolesService.getCreatorRole())
                .likesFilled(false)
                .build());
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
                .get("/i/{queueId}", queueId).then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .extract().as(QueueQueryDto.class);

        assertNull(created.getCurrent());
        assertEquals(0, created.getQueuedSongs().size());
    }

    @Test
    void shouldEnqueueYoutubeSong() throws MalformedURLException {
        String videoId = YoutubeApiVideosClientMock.RESPONSES.keySet().iterator().next();

        QueuedSongShortQueryDto created = given()
                .auth().basic("zanna", "zanna")
                .contentType(ContentType.JSON)
                .body(YoutubeSongAddDto.builder().videoId(videoId).build())
                .post("/i/{queueId}/queue", queueId).then()
                .statusCode(StatusCode.CREATED)
                .contentType(ContentType.JSON)
                .extract().as(QueuedSongShortQueryDto.class);

        YoutubeVideoDataDto videoData = YoutubeApiVideosClientMock.RESPONSES.get(videoId).unwrapSingle();

        assertEquals(0, created.getLikes());
        assertEquals(videoData.getSnippet().getTitle(), created.getName());

        Queue queue = Queue.findById(queueId);

        assertEquals(1, queue.getQueuedSongs().size());

        QueuedSong enqueued = queue.getQueuedSongs().get(0);

        assertEquals(0, enqueued.getLikes());
        assertInstanceOf(YoutubeSong.class, enqueued.getSong());

        YoutubeSong song = (YoutubeSong) enqueued.getSong();

        assertEquals(videoId, song.getVideoId());
        assertEquals(created.getName(), song.getName());
    }

    @Test
    void shouldSortEnqueuedByTime() throws InterruptedException {
        String[] videoIds = YoutubeApiVideosClientMock.RESPONSES.keySet().toArray(String[]::new);

        for (var videoId : videoIds) {
            given()
                    .auth().basic("zanna", "zanna")
                    .contentType(ContentType.JSON)
                    .body(YoutubeSongAddDto.builder().videoId(videoId).build())
                    .post("/i/{queueId}/queue", queueId).then()
                    .statusCode(StatusCode.CREATED);
            // Minimal separation between requests to ensure they are correctly sorted
            Thread.sleep(10);
        }

        Queue queue = Queue.findById(queueId);

        assertEquals(videoIds.length, queue.getQueuedSongs().size());

        assertAll(Streams.zip(
                Arrays.stream(videoIds),
                queue.getQueuedSongs().stream().map(queuedSong -> {
                    YoutubeSong song = (YoutubeSong) queuedSong.getSong();
                    return song.getVideoId();
                }),
                (given, inserted) -> () -> assertEquals(given, inserted)));
    }
}
