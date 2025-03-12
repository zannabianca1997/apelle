package io.github.zannabianca1997.apelle;

import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

import org.eclipse.microprofile.config.inject.ConfigProperty;

@QuarkusTest
@TestHTTPEndpoint(VersionResource.class)
class VersionResourceTest {

    @ConfigProperty(name = "quarkus.application.version")
    String version;

    @Test
    void shouldReturnVersion() {
        given()
                .when().get("/")
                .then()
                .statusCode(200)
                .body(is(version));
    }

}