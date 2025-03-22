package io.github.zannabianca1997.apelle.queues;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import io.quarkus.security.Authenticated;
import io.quarkus.security.PermissionChecker;
import io.quarkus.security.PermissionsAllowed;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.inject.Inject;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueued;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;

@Path("/queues/{queueId}/queue/{songId}")
@Tag(name = "Queued song", description = "Interaction with a queued song")
@Authenticated
public class QueuedSongResource {

    @Inject
    private SongMapper songMapper;
    @Inject
    private QueueUserService queueUserService;

    @GET
    @Operation(summary = "Get the queued song", description = """
            Get the full state of the queued song, with all data.

            TODO: Add query parameters to ask for thumbnails.""")
    @APIResponse(responseCode = "200", description = "The queued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongQueryDto.class))
    })
    public QueuedSongQueryDto get(UUID queueId, UUID songId) throws QueueNotFoundException, SongNotQueued {
        QueuedSong queuedSong = QueuedSong.findById(queueId, songId);
        if (queuedSong == null) {
            if (!Queue.exists(queueId)) {
                throw new QueueNotFoundException(queueId);
            }
            throw new SongNotQueued(queueId, songId);
        }
        return songMapper.toDto(queuedSong);
    }

    @POST
    @Path("/likes")
    @Operation(summary = "Add a like to the song", description = """
            Add a like to the song, pushing it upwards in the queue.

            If the maximum number of likes was already reached, the oldest like will be removed.
            """)
    @PermissionsAllowed("queued-song-like")
    public void like(UUID queueId, UUID songId) {
        // TODO
    }

    @PermissionChecker("queued-song-like")
    boolean canLike(UUID queueId) {
        try {
            return queueUserService.getCurrent(queueId).getMaxLikes() > 0;
        } catch (QueueNotFoundException e) {
            return true;
        }
    }
}
