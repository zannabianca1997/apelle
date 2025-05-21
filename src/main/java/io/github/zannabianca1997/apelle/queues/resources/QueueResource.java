package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.ResponseHeader;
import org.jboss.resteasy.reactive.ResponseStatus;
import org.jboss.resteasy.reactive.RestQuery;
import org.jboss.resteasy.reactive.RestStreamElementType;
import org.jboss.resteasy.reactive.RestResponse.StatusCode;

import io.quarkus.security.Authenticated;
import io.smallrye.common.annotation.Blocking;
import io.smallrye.mutiny.Multi;
import jakarta.annotation.security.PermitAll;
import jakarta.enterprise.context.Initialized;
import jakarta.enterprise.context.RequestScoped;
import jakarta.enterprise.event.Observes;
import jakarta.transaction.TransactionScoped;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Context;
import jakarta.ws.rs.core.EntityTag;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Request;
import jakarta.ws.rs.core.Response;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongShortQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueEventDto;
import io.github.zannabianca1997.apelle.queues.dtos.events.QueueStateEventDto;
import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueueException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongAlreadyQueuedException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueuedException;
import io.github.zannabianca1997.apelle.queues.mappers.EventMapper;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.services.QueueService;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;
import io.github.zannabianca1997.apelle.queues.services.SongService;
import io.github.zannabianca1997.apelle.queues.services.QueueService.EnqueueResult;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.YoutubeVideoNotFoundException;

@Authenticated
@RequestScoped
public class QueueResource {
    private final QueueMapper queueMapper;
    private final SongMapper songMapper;
    private final EventMapper eventMapper;

    private final QueueService queueService;
    private final SongService songService;
    private final QueueUserService queueUserService;

    private final QueueSongResource queueSongResource;
    private final QueueUserResource queueUserResource;

    public QueueResource(
            final QueueMapper queueMapper,
            final SongMapper songMapper,
            final EventMapper eventMapper,
            final QueueService queueService,
            final SongService songService,
            final QueueUserService queueUserService,
            final QueueSongResource queueSongResource,
            final QueueUserResource queueUserResource) {
        this.queueMapper = queueMapper;
        this.songMapper = songMapper;
        this.eventMapper = eventMapper;
        this.queueService = queueService;
        this.songService = songService;
        this.queueUserService = queueUserService;
        this.queueSongResource = queueSongResource;
        this.queueUserResource = queueUserResource;
    }

    private Queue queue = null;
    private QueueUser current = null;

    public QueueResource of(final Queue queue) {
        this.queue = queue;
        this.current = queueUserService.getCurrent(queue);
        return this;
    }

    @PermitAll
    void onBeginTransaction(@Observes @Initialized(TransactionScoped.class) final Object event) {
        if (queue != null)
            queue = Queue.getEntityManager().merge(queue);
        if (current != null)
            current = QueueUser.getEntityManager().merge(current);
    }

    @GET
    @Operation(summary = "Get the queue state", description = "Get the queue state, with both the currently playing song and the list of songs to play next")
    @APIResponse(responseCode = "200", description = "The queue state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public QueueQueryDto get() {
        return queueMapper.toDto(queue, queuedSong -> queueUserService.likes(current, queuedSong));
    }

    @POST
    @Path("/queue")
    @Operation(summary = "Add a song to the queue", description = "Add a song to the queue, with no likes.")
    @APIResponse(responseCode = "201", description = "The enqueued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongShortQueryDto.class))
    })
    @ResponseStatus(StatusCode.CREATED)
    @Transactional
    @Tag(name = "Queued song")
    public QueuedSongShortQueryDto enqueue(final SongAddDto songAddDto, @RestQuery("autolike") final Boolean autolike)
            throws BadYoutubeApiResponseException, SongAlreadyQueuedException, ActionNotPermittedException,
            YoutubeVideoNotFoundException {
        final Song song = songService.fromDto(songAddDto);
        final EnqueueResult enqueued = queueService.enqueue(queue, song, autolike);
        return songMapper.toShortDto(enqueued.queuedSong(), enqueued.autolikes());
    }

    @Path("/queue/{songId}")
    public QueueSongResource queueSong(final UUID songId) throws SongNotQueuedException {
        final QueuedSong song = queueService.getQueuedSong(queue, songId);
        return queueSongResource.of(song, current);
    }

    @POST
    @Path("/start")
    @Operation(summary = "Start playing", description = "Start playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started")
    @APIResponse(responseCode = "412", description = "The player state id did not match")
    @Transactional
    public Response start(@Context final Request request)
            throws CantPlayEmptyQueueException, ActionNotPermittedException {
        final var preconditions = request
                .evaluatePreconditions(new EntityTag(queue.getPlayerStateId().toString(), true));
        if (preconditions != null) {
            return preconditions.build();
        }

        queueService.start(queue);
        return Response.noContent().build();
    }

    @POST
    @Path("/stop")
    @Operation(summary = "Stop playing", description = "Stop playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started")
    @APIResponse(responseCode = "412", description = "The player state id did not match")
    @Transactional
    public Response stop(@Context final Request request) throws ActionNotPermittedException {
        final var preconditions = request
                .evaluatePreconditions(new EntityTag(queue.getPlayerStateId().toString(), true));
        if (preconditions != null) {
            return preconditions.build();
        }

        queueService.stop(queue);
        return Response.noContent().build();
    }

    @POST
    @Path("/next")
    @Operation(summary = "Start playing the next song", description = """
            Start the next song in the queue.
            The current one will be requeued as the last one, with no likes.""")
    @APIResponse(responseCode = "204", description = "The music started")
    @APIResponse(responseCode = "412", description = "The player state id did not match")
    @Transactional
    public Response next(@Context final Request request)
            throws CantPlayEmptyQueueException, ActionNotPermittedException {
        final var preconditions = request
                .evaluatePreconditions(new EntityTag(queue.getPlayerStateId().toString(), true));
        if (preconditions != null) {
            return preconditions.build();
        }

        queueService.next(queue);
        return Response.noContent().build();
    }

    @Path("/users/i/{userId}")
    public QueueUserResource userById(final UUID userId) throws UserNotFoundByIdException {
        final QueueUser otherUser = queueUserService.getById(queue, userId);
        return queueUserResource.of(otherUser);
    }

    @Path("/users/n/{userName}")
    public QueueUserResource userByName(final String userName) throws UserNotFoundByNameException {
        final QueueUser otherUser = queueUserService.getByName(queue, userName);
        return queueUserResource.of(otherUser);
    }

    @Path("/users/me")
    public QueueUserResource userByName() {
        return queueUserResource.of(current);
    }

    @DELETE
    @Operation(summary = "Delete the queue", description = "Delete the queue permanently")
    @APIResponse(responseCode = "204", description = "The queue was deleted.")
    @Transactional
    public void delete() throws ActionNotPermittedException {
        queueService.delete(queue);
    }

    @GET
    @RestStreamElementType(MediaType.APPLICATION_JSON)
    @Operation(summary = "Obtain a stream of events regarding this queue.")
    @Path("/events")
    // Signal to NGINX that the messages should reach the frontend as soon as
    // possible
    @ResponseHeader(name = "X-Accel-Buffering", value = "no")
    @Blocking
    public Multi<QueueEventDto> events() {
        return Multi.createFrom().<QueueEventDto>item(QueueStateEventDto.builder().queue(get()).build())
                .onCompletion().switchTo(queueService.events(queue, current).map(eventMapper::toDto));
    }
}
