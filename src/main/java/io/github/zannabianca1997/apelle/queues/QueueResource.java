package io.github.zannabianca1997.apelle.queues;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;

import io.quarkus.security.Authenticated;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Response.Status;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.services.QueueService;
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
    QueueService queueService;
    @Inject
    SongService songService;

    @GET
    @Operation(summary = "Get the queue state", description = "Get the queue state, with both the currently playing song and the list of songs to play next")
    @APIResponse(responseCode = "200", description = "The queue state", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueQueryDto.class))
    })
    public QueueQueryDto get(UUID queueId) throws QueueNotFoundException {
        Queue queue = queueService.get(queueId);
        return queueMapper.toDto(queue);
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
        Song song = songService.fromDto(songAddDto);
        QueuedSong enqueued = queueService.enqueue(queueId, song);
        return RestResponse.status(Status.CREATED, songMapper.toDto(enqueued));
    }

    @POST
    @Path("/play")
    @Operation(summary = "Start playing", description = "Start playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started", content = {})
    @Transactional
    public void play(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        queueService.play(queueId);
    }

    @POST
    @Path("/stop")
    @Operation(summary = "Stop playing", description = "Stop playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started", content = {})
    @Transactional
    public void stop(UUID queueId)
            throws QueueNotFoundException {
        queueService.stop(queueId);
    }

    @POST
    @Path("/next")
    @Operation(summary = "Start playing the next song", description = """
            Start the next song in the queue.
            The current one will be requeued as the last one, with no likes.""")
    @APIResponse(responseCode = "204", description = "The music started", content = {})
    @Transactional
    public void next(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        queueService.next(queueId);
    }
}
