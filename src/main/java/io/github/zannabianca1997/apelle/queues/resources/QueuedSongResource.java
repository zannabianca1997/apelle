package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.parameters.Parameter;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import io.quarkus.security.Authenticated;
import io.quarkus.security.PermissionChecker;
import io.quarkus.security.PermissionsAllowed;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.inject.Inject;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DefaultValue;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.QueryParam;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueued;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.services.QueueService;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;

@Path("/queues/{queueId}/queue/{songId}")
@Tag(name = "Queued song", description = "Interaction with a queued song")
@Authenticated
public class QueuedSongResource {

    @Inject
    SongMapper songMapper;
    @Inject
    QueueUserService queueUserService;
    @Inject
    QueueService queueService;

    @GET
    @Operation(summary = "Get the queued song", description = """
            Get the full state of the queued song, with all data.

            TODO: Add query parameters to ask for thumbnails.""")
    @APIResponse(responseCode = "200", description = "The queued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongQueryDto.class))
    })
    public QueuedSongQueryDto get(UUID queueId, UUID songId) throws QueueNotFoundException, SongNotQueued {
        return songMapper.toDto(queueService.getQueuedSong(queueId, songId));
    }

    @POST
    @Path("/likes")
    @Operation(summary = "Add a like to the song", description = """
            Add a like to the song, pushing it upwards in the queue.

            If the maximum number of likes was already reached, the oldest like will be removed.
            This will happen trasparently even if a number of likes larger than available is specified,
            effectively removing all likes and moving them to the song.""")
    @Parameter(name = "count", description = "How many time to like the song. If negative, nothing will happen.")
    @PermissionsAllowed("queue-like-song")
    @Transactional
    public void like(UUID queueId, UUID songId,
            @QueryParam("count") @DefaultValue("1") short count)
            throws SongNotQueued, QueueNotFoundException {
        QueuedSong queuedSong = queueService.getQueuedSong(queueId, songId);
        queueService.like(queuedSong, queueUserService.getCurrent(queueId), count);
    }

    @PermissionChecker("queue-like-song")
    boolean canNextSong(SecurityIdentity identity, UUID queueId) {
        QueueUser queueUser;
        try {
            queueUser = queueUserService.getCurrent(queueId);
        } catch (QueueNotFoundException e) {
            return true;
        }
        return queueUser.getPermissions().likeSong();
    }
}
