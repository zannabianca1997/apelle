# `apelle`

> A communist music queue

`apelle` is a backend for handling a shared music queue.
Users can insert songs in the queues, and upvote them to push them upward. `apelle`
will track the position of each song in the queue, and the position of the currently
playing song.

It also fetch the song data from the sources (for now, only Youtube is supported).
Users provides only the minimal necessary to identify the song (e.g. the youtube video ID).

## Needed API keys

`apelle` loads the song data from external sources. API keys need to be provided for each one of them to work.

### Google

`apelle` needs access to Google Youtube API to fetch the video data. Provide the key in the property `apelle.youtube.api.key`.
For example, using a `.env` in the project root:

```env
apelle.youtube.api.key=<your-key-here>
```

The key need to be able to query the youtube API v3.

## Running the application in dev mode

You can run your application in dev mode that enables live coding using:

```shell script
./gradlew quarkusDev -Dvertx.disableURIValidation=true
```

You can then found the web UI at <http://localhost:8080/>. Hot reloading is enabled both on the frontend and the backend.

You can also find the dev UI at <http://localhost:8080/q/dev/>, and the Swagged OpenAPI documentation at <http://localhost:8080/q/swagger-ui/>.

The `-Dvertx.disableURIValidation=true` is needed to handle sveltekit dynamic routes. It should not be enabled in production.

## Packaging and running the application

The application can be packaged using:

```shell script
./gradlew build
```

It produces the `quarkus-run.jar` file in the `build/quarkus-app/` directory.
Be aware that it’s not an _über-jar_ as the dependencies are copied into the `build/quarkus-app/lib/` directory.

The application is now runnable using `java -jar build/quarkus-app/quarkus-run.jar`.

If you want to build an _über-jar_, execute the following command:

```shell script
./gradlew build -Dquarkus.package.jar.type=uber-jar
```

The application, packaged as an _über-jar_, is now runnable using `java -jar build/*-runner.jar`.

## Creating a native executable

You can create a native executable using:

```shell script
./gradlew build -Dquarkus.native.enabled=true -Dquarkus.package.jar.enabled=false
```

Or, if you don't have GraalVM installed, you can run the native executable build in a container using:

```shell script
./gradlew build -Dquarkus.native.enabled=true -Dquarkus.native.container-build=true -Dquarkus.package.jar.enabled=false
```

You can then execute your native executable with: `./build/apelle-0.0.1-runner`

If you want to learn more about building native executables, please consult <https://quarkus.io/guides/gradle-tooling>.
