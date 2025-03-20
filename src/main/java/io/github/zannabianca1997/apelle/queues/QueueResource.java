package io.github.zannabianca1997.apelle.queues;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;

import io.quarkus.security.Authenticated;
import io.smallrye.mutiny.Uni;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.eventbus.EventBus;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Response.Status;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueuePlayEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.services.SongService;
import io.github.zannabianca1997.apelle.youtube.exceptions.BadYoutubeApiResponse;

@Path("/queues/{queueId}")
@Tag(name = "Queue", description = "Direct management of the queue")
@Authenticated
public class QueueResource {

    @Inject
    QueueMapper queueMapper;
    @Inject
    SongMapper songMapper;
    @Inject
    SongService songService;
    @Inject
    EventBus eventBus;

    /**
     * Send a queue event.
     * 
     * The event is published on the address equal to the queue ID.
     * 
     * @param event The event to publish
     */
    private void publish(QueueEvent event) {
        eventBus.publish(event.getQueueId().toString(), JsonObject.mapFrom(event));
    }

    private Queue getQueue(UUID queueId) throws QueueNotFoundException {
        Queue queue = Queue.findById(queueId);
        if (queue == null) {
            throw new QueueNotFoundException(queueId);
        }
        return queue;
    }

    /**
     * Schedule the queue to be stopped when it finished
     * 
     * @param queue The queue to stop
     */
    private void scheduleStopAtEnd(Queue queue) {
        final UUID queueId = queue.getId();
        // Fire when the song would end
        Uni<Boolean> songEnded = Uni.createFrom().voidItem()
                .onItem().delayIt().by(queue.getCurrent().timeLeft())
                .replaceWith(false);
        // Fire if something stop the song
        Uni<Boolean> stopEvent = eventBus.<JsonObject>consumer(queueId.toString())
                .toMulti()
                .map(jsonObject -> jsonObject.body().mapTo(QueueEvent.class))
                .filter(event -> event instanceof QueueStopEvent)
                .onItem().castTo(QueueStopEvent.class)
                .toUni().replaceWith(true);
        // On song completion, if nothing stopped it before, stop the song
        Uni.combine().any().of(songEnded, stopEvent)
                .subscribe().with(stopped -> {
                    if (!stopped) {
                        try {
                            stop(queueId);
                        } catch (QueueNotFoundException e) {
                            // Queue was deleted, nothing to do
                        }
                    }
                });
    }

    @GET
    @Operation(summary = "Get the queue state", description = "Get the queue state, with both the currently playing song and the list of songs to play next")
    @APIResponse(responseCode = "200", description = "The queue state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public QueueQueryDto get(UUID queueId) throws QueueNotFoundException {
            return queueMapper.toDto(getQueue(queueId));
    }

    @POST
    @Path("/queued-songs")
    @Operation(summary = "Add a song to the queue", description = "Add a song to the queue, with no likes.")
    @APIResponse(responseCode = "201", description = "The enqueued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongQueryDto.class))
    })
    @Transactional
    public RestResponse<QueuedSongQueryDto> enqueue(UUID queueId, SongAddDto songAddDto)
            throws QueueNotFoundException, BadYoutubeApiResponse {
        var song = songService.fromDto(songAddDto);
        var queue = getQueue(queueId);

        var enqueued = queue.enqueue(song);
        publish(QueueEnqueueEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());

            return RestResponse.status(Status.CREATED, songMapper.toDto(enqueued));
    }

    @POST
    @Path("/play")
    @Operation(summary = "Start playing", description = "Start playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started", content = {})
    @Transactional
    public void play(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        Queue queue = getQueue(queueId);
        boolean startedNow = queue.play();
        if (startedNow) {
            publish(QueuePlayEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());
            scheduleStopAtEnd(queue);
        }
    }

    @POST
    @Path("/stop")
    @Operation(summary = "Stop playing", description = "Stop playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started", content = {})
    @Transactional
    public void stop(UUID queueId)
            throws QueueNotFoundException {
        Queue queue = getQueue(queueId);
        boolean stoppedNow = queue.stop();
        if (stoppedNow) {
            publish(QueueStopEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());
    }
}
