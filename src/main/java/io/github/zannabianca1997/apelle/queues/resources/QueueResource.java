package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;
import org.jboss.resteasy.reactive.RestStreamElementType;

import io.quarkus.security.Authenticated;
import io.smallrye.mutiny.Multi;
import jakarta.annotation.security.PermitAll;
import jakarta.enterprise.context.Initialized;
import jakarta.enterprise.context.RequestScoped;
import jakarta.enterprise.event.Observes;
import jakarta.inject.Inject;
import jakarta.transaction.TransactionScoped;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.MediaType;
import jakarta.ws.rs.core.Response.Status;
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
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponseException;
import io.github.zannabianca1997.apelle.youtube.exceptions.YoutubeVideoNotFoundException;

@Authenticated
@RequestScoped
public class QueueResource {
    @Inject
    QueueMapper queueMapper;
    @Inject
    SongMapper songMapper;
    @Inject
    EventMapper eventMapper;

    @Inject
    QueueService queueService;
    @Inject
    SongService songService;
    @Inject
    QueueUserService queueUserService;

    @Inject
    QueueSongResource queueSongResource;
    @Inject
    QueueUserResource queueUserResource;

    Queue queue = null;

    public QueueResource of(Queue queue) {
        this.queue = queue;
        return this;
    }

    @PermitAll
    void onBeginTransaction(@Observes @Initialized(TransactionScoped.class) Object event) {
        if (queue != null)
            queue = Queue.getEntityManager().merge(queue);
    }

    @GET
    @Operation(summary = "Get the queue state", description = "Get the queue state, with both the currently playing song and the list of songs to play next")
    @APIResponse(responseCode = "200", description = "The queue state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public QueueQueryDto get() {
        return queueMapper.toDto(queue);
    }

    @POST
    @Path("/queue")
    @Operation(summary = "Add a song to the queue", description = "Add a song to the queue, with no likes.")
    @APIResponse(responseCode = "201", description = "The enqueued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongShortQueryDto.class))
    })
    @Transactional
    @Tag(name = "Queued song")
    public RestResponse<QueuedSongShortQueryDto> enqueue(SongAddDto songAddDto)
            throws BadYoutubeApiResponseException, SongAlreadyQueuedException, ActionNotPermittedException,
            YoutubeVideoNotFoundException {
        Song song = songService.fromDto(songAddDto);
        QueuedSong enqueued = queueService.enqueue(queue, song);
        return RestResponse.status(Status.CREATED, songMapper.toShortDto(enqueued));
    }

    @Path("/queue/{songId}")
    public QueueSongResource queueSong(UUID songId) throws SongNotQueuedException {
        QueuedSong song = queueService.getQueuedSong(queue, songId);
        QueueUser user = queueUserService.getCurrent(queue);
        return queueSongResource.of(song, user);
    }

    @POST
    @Path("/start")
    @Operation(summary = "Start playing", description = "Start playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started")
    @Transactional
    public void start() throws CantPlayEmptyQueueException, ActionNotPermittedException {
        queueService.start(queue);
    }

    @POST
    @Path("/stop")
    @Operation(summary = "Stop playing", description = "Stop playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started")
    @Transactional
    public void stop() throws ActionNotPermittedException {
        queueService.stop(queue);
    }

    @POST
    @Path("/next")
    @Operation(summary = "Start playing the next song", description = """
            Start the next song in the queue.
            The current one will be requeued as the last one, with no likes.""")
    @APIResponse(responseCode = "204", description = "The music started")
    @Transactional
    public void next() throws CantPlayEmptyQueueException, ActionNotPermittedException {
        queueService.next(queue);
    }

    @Path("/users/i/{userId}")
    public QueueUserResource userById(UUID userId) throws UserNotFoundByIdException {
        QueueUser otherUser = queueUserService.getById(queue, userId);
        return queueUserResource.of(otherUser);
    }

    @Path("/users/n/{userName}")
    public QueueUserResource userByName(String userName) throws UserNotFoundByNameException {
        QueueUser otherUser = queueUserService.getByName(queue, userName);
        return queueUserResource.of(otherUser);
    }

    @Path("/users/me")
    public QueueUserResource userByName() {
        QueueUser user = queueUserService.getCurrent(queue);
        return queueUserResource.ofMe(user);
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
    public Multi<QueueEventDto> events() {
        return Multi.createFrom().<QueueEventDto>item(QueueStateEventDto.builder().queue(get()).build())
                .onCompletion().switchTo(queueService.events(queue).map(eventMapper::toDto));
    }
}
