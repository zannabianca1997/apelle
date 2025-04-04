package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import org.jboss.resteasy.reactive.RestResponse;

import io.quarkus.security.Authenticated;
import io.quarkus.security.PermissionChecker;
import io.quarkus.security.PermissionsAllowed;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.core.Response.Status;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongShortQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongAlreadyQueued;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.services.QueueService;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;
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
    @Inject
    QueueUserService queueUserService;

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
    @Path("/queue")
    @Operation(summary = "Add a song to the queue", description = "Add a song to the queue, with no likes.")
    @APIResponse(responseCode = "201", description = "The enqueued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongShortQueryDto.class))
    })
    @Transactional
    @PermissionsAllowed("queue-enqueue")
    @Tag(name = "Queued song")
    public RestResponse<QueuedSongShortQueryDto> enqueue(UUID queueId, SongAddDto songAddDto)
            throws QueueNotFoundException, BadYoutubeApiResponse, SongAlreadyQueued {
        Song song = songService.fromDto(songAddDto);
        QueuedSong enqueued = queueService.enqueue(queueId, song);
        return RestResponse.status(Status.CREATED, songMapper.toShortDto(enqueued));
    }

    @POST
    @Path("/start")
    @Operation(summary = "Start playing", description = "Start playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started")
    @Transactional
    @PermissionsAllowed("queue-start-song")
    public void start(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        queueService.start(queueId);
    }

    @POST
    @Path("/stop")
    @Operation(summary = "Stop playing", description = "Stop playing music from the queue.")
    @APIResponse(responseCode = "204", description = "The music started")
    @Transactional
    @PermissionsAllowed("queue-stop-song")
    public void stop(UUID queueId)
            throws QueueNotFoundException {
        queueService.stop(queueId);
    }

    @POST
    @Path("/next")
    @Operation(summary = "Start playing the next song", description = """
            Start the next song in the queue.
            The current one will be requeued as the last one, with no likes.""")
    @APIResponse(responseCode = "204", description = "The music started")
    @Transactional
    @PermissionsAllowed("queue-next-song")
    public void next(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        queueService.next(queueId);
    }

    @PermissionChecker("queue-start-song")
    boolean canStartSong(SecurityIdentity identity, UUID queueId) {
        QueueUser queueUser;
        try {
            queueUser = queueUserService.getCurrent(queueId);
        } catch (QueueNotFoundException e) {
            return true;
        }
        return queueUser.getPermissions().startSong();
    }

    @PermissionChecker("queue-stop-song")
    boolean canStopSong(SecurityIdentity identity, UUID queueId) {
        QueueUser queueUser;
        try {
            queueUser = queueUserService.getCurrent(queueId);
        } catch (QueueNotFoundException e) {
            return true;
        }
        return queueUser.getPermissions().stopSong();
    }

    @PermissionChecker("queue-next-song")
    boolean canNextSong(SecurityIdentity identity, UUID queueId) {
        QueueUser queueUser;
        try {
            queueUser = queueUserService.getCurrent(queueId);
        } catch (QueueNotFoundException e) {
            return true;
        }
        return queueUser.getPermissions().nextSong();
    }

    @PermissionChecker("queue-enqueue")
    boolean canEnqueue(SecurityIdentity identity, UUID queueId) {
        QueueUser queueUser;
        try {
            queueUser = queueUserService.getCurrent(queueId);
        } catch (QueueNotFoundException e) {
            return true;
        }
        return queueUser.getPermissions().enqueue();
    }
}
