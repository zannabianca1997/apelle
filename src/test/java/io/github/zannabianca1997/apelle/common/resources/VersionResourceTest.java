package io.github.zannabianca1997.apelle.common.resources;

import io.quarkus.test.common.http.TestHTTPEndpoint;
import io.quarkus.test.junit.QuarkusTest;

import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import static io.restassured.RestAssured.given;
import static org.hamcrest.CoreMatchers.is;

import org.eclipse.microprofile.config.inject.ConfigProperty;

@QuarkusTest
@TestHTTPEndpoint(VersionResource.class)
@Tag("common")
class VersionResourceTest {

    private final String version;

    public VersionResourceTest(@ConfigProperty(name = "quarkus.application.version") final String version) {
        this.version = version;
    }

    @Test
    void shouldReturnVersion() {
        given()
                .when().get("/")
                .then()
                .statusCode(200)
                .body(is(version));
    }

}