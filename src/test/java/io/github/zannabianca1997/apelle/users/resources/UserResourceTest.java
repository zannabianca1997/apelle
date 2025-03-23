package io.github.zannabianca1997.apelle.users.resources;

import static io.restassured.RestAssured.given;
import static org.hamcrest.Matchers.*;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertNull;

import java.util.UUID;

import org.jboss.resteasy.reactive.RestResponse.StatusCode;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import io.github.zannabianca1997.apelle.users.dtos.UserCreateDto;
import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import io.github.zannabianca1997.apelle.users.mappers.UserMapper;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;
import io.quarkus.narayana.jta.QuarkusTransaction;
import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import io.restassured.http.ContentType;
import io.restassured.response.Response;
import jakarta.inject.Inject;
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

    Response createUserRequest(String name, String password) {
        return given()
                .auth().none()
                .contentType(ContentType.JSON)
                .body(UserCreateDto.builder()
                        .name(name)
                        .password(password)
                        .build())
                .post();
    }

    ApelleUser createUser(String name, String password) {
        var user = ApelleUser.builder()
                .name(name).password(password)
                .role(ApelleUserRole.USER)
                .build();
        QuarkusTransaction.requiringNew()
                .run(user::persist);
        return user;
    }

    @Test
    void shouldCreateUser() {
        UserQueryDto created = createUserRequest("zanna", "zanna").then()
                .statusCode(StatusCode.CREATED)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals("zanna", created.getName());
    }

    @Test
    void shouldNotCreateDoubleUser() {
        createUser("zanna", "zanna");

        createUserRequest("zanna", "other pass").then()
                .statusCode(StatusCode.CONFLICT);
    }

    @Inject
    private UserMapper userMapper;

    @Test
    void shouldReturnCurrentUser() {
        ApelleUser created = createUser("zanna", "zanna");

        UserQueryDto found = given()
                .auth().basic("zanna", "zanna")
                .get("/me")
                .then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals(userMapper.toDto(created), found);
    }

    @Test
    void shouldDeleteCurrentUser() {
        UUID createdId = createUser("zanna", "zanna").getId();

        given()
                .auth().basic("zanna", "zanna")
                .delete("/me")
                .then()
                .statusCode(StatusCode.NO_CONTENT);

        assertNull(ApelleUser.findById(createdId));
    }

    @Test
    void shouldFindUserById() {
        ApelleUser created = createUser("zanna", "zanna");

        UserQueryDto found = given()
                .auth().basic("zanna", "zanna")
                .get("/i/{id}", created.getId())
                .then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals(userMapper.toDto(created), found);
    }

    @Test
    void shouldNotDeleteOtherUserById() {
        createUser("zanna", "zanna");
        UUID createdId = createUser("other", "other_password").getId();

        given()
                .auth().basic("zanna", "zanna")
                .delete("/i/{createdId}", createdId)
                .then()
                .statusCode(StatusCode.FORBIDDEN);

        assertNotNull(ApelleUser.findById(createdId));
    }

    @Test
    void shouldFindUserByName() {
        ApelleUser created = createUser("zanna", "zanna");

        UserQueryDto found = given()
                .auth().basic("zanna", "zanna")
                .get("/n/{name}", created.getName())
                .then()
                .statusCode(StatusCode.OK)
                .contentType(ContentType.JSON)
                .body("$", not(hasKey("password")))
                .extract().as(UserQueryDto.class);

        assertEquals(userMapper.toDto(created), found);
    }

    @Test
    void shouldNotDeleteOtherUserByName() {
        createUser("zanna", "zanna");
        UUID createdId = createUser("other", "other_password").getId();

        given()
                .auth().basic("zanna", "zanna")
                .delete("/n/other")
                .then()
                .statusCode(StatusCode.FORBIDDEN);

        assertNotNull(ApelleUser.findById(createdId));
    }
}
