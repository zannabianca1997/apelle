package io.github.zannabianca1997.apelle.queues;

import static io.restassured.RestAssured.given;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

import org.jboss.resteasy.reactive.RestResponse.StatusCode;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;
import jakarta.transaction.Transactional;

@QuarkusTest
@TestHTTPEndpoint(QueuesResource.class)
@Tag("queues")
class QueuesResourceTest {
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

    @BeforeEach
    @Transactional
    void deleteAllQueues() {
        Queue.deleteAll();
    }

    @Test
    void shouldCreateQueue() {
        QueueQueryDto created = given()
                .auth().basic("zanna", "zanna")
                .post().then()
                .statusCode(StatusCode.CREATED)
                .contentType(ContentType.JSON)
                .extract().as(QueueQueryDto.class);

        assertNull(created.getCurrent());
        assertEquals(0, created.getQueuedSongs().size());

        assertEquals(1, Queue.findAll().count());
        Queue createdEntity = Queue.findById(created.getId());

        assertNotNull(createdEntity);
        assertNull(createdEntity.getCurrent());
        assertEquals(0, createdEntity.getQueuedSongs().size());
    }

    @Test
    void shouldNeedAuthentication() {
        given()
                .auth().none()
                .post().then()
                .statusCode(StatusCode.UNAUTHORIZED);

        assertEquals(0, Queue.findAll().count());
    }
}
