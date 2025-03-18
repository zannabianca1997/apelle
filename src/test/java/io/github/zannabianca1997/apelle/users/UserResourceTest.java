package io.github.zannabianca1997.apelle.users;

import static io.restassured.RestAssured.given;
import static org.hamcrest.Matchers.*;
import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.UUID;

import org.jboss.resteasy.reactive.RestResponse.StatusCode;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;
import io.restassured.response.Response;
import jakarta.transaction.Transactional;

@QuarkusTest
@TestHTTPEndpoint(UsersResource.class)
@Tag("auth")
class UserResourceTest {

    @BeforeEach
    @Transactional
    void deleteAllUsers() {
        ApelleUser.deleteAll();
    }

    Response createUser(String name, String password) {
        return given()
                .auth().none()
                .contentType(ContentType.JSON)
                .body(UserCreateDto.builder()
                        .name(name)
                        .password(password)
                        .build())
                .post();
    }

    @Test
    void shouldCreateUser() {
        UserQueryDto created = createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals("zanna", created.getName());
    }

    @Test
    void shouldNotCreateDoubleUser() {
        createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED);
        createUser("zanna", "other pass").then()
                .log().body()
                .statusCode(StatusCode.CONFLICT);
    }

    @Test
    void shouldReturnCurrentUser() {
        UserQueryDto created = createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED)
                .extract().as(UserQueryDto.class);

        UserQueryDto found = given()
                .auth().basic("zanna", "zanna")
                .get("/me")
                .then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals(created, found);
    }

    @Test
    void shouldDeleteCurrentUser() {
        createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED);

        given()
                .auth().basic("zanna", "zanna")
                .delete("/me")
                .then()
                .statusCode(StatusCode.NO_CONTENT);

        given()
                .auth().basic("zanna", "zanna")
                .get("/me")
                .then()
                .statusCode(StatusCode.UNAUTHORIZED);
    }

    @Test
    void shouldFindUserById() {
        UserQueryDto created = createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED)
                .extract().as(UserQueryDto.class);

        UserQueryDto found = given()
                .auth().basic("zanna", "zanna")
                .get("/i/{id}", created.getId())
                .then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals(created, found);
    }

    @Test
    void shouldNotDeleteOtherUserById() {
        createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED);
        UUID createdId = createUser("other", "other_password").then()
                .statusCode(StatusCode.CREATED)
                .extract().jsonPath().getUUID("id");

        given()
                .auth().basic("zanna", "zanna")
                .delete("/i/{createdId}", createdId)
                .then()
                .statusCode(StatusCode.FORBIDDEN);

        given()
                .auth().basic("other", "other_password")
                .get("/me")
                .then()
                .statusCode(StatusCode.OK);
    }

    @Test
    void shouldFindUserByName() {
        UserQueryDto created = createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED)
                .extract().as(UserQueryDto.class);

        UserQueryDto found = given()
                .auth().basic("zanna", "zanna")
                .get("/n/{id}", created.getName())
                .then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals(created, found);
    }

    @Test
    void shouldNotDeleteOtherUserByName() {
        createUser("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED);
        createUser("other", "other_password").then()
                .statusCode(StatusCode.CREATED)
                .extract().jsonPath().getUUID("id");

        given()
                .auth().basic("zanna", "zanna")
                .delete("/n/other")
                .then()
                .statusCode(StatusCode.FORBIDDEN);

        given()
                .auth().basic("other", "other_password")
                .get("/me")
                .then()
                .statusCode(StatusCode.OK);
    }
}
