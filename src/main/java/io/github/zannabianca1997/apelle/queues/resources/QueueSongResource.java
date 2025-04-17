package io.github.zannabianca1997.apelle.queues.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.parameters.Parameter;
import org.eclipse.microprofile.openapi.annotations.responses.APIResponse;
import org.eclipse.microprofile.openapi.annotations.tags.Tag;
import io.quarkus.security.Authenticated;
import jakarta.annotation.security.PermitAll;
import jakarta.enterprise.context.Initialized;
import jakarta.enterprise.context.RequestScoped;
import jakarta.enterprise.event.Observes;
import jakarta.inject.Inject;
import jakarta.transaction.TransactionScoped;
import jakarta.transaction.Transactional;
import jakarta.ws.rs.DELETE;
import jakarta.ws.rs.DefaultValue;
import jakarta.ws.rs.GET;
import jakarta.ws.rs.POST;
import jakarta.ws.rs.Path;
import jakarta.ws.rs.QueryParam;
import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.services.QueueService;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;

@Tag(name = "Queued song", description = "Interaction with a queued song")
@Authenticated
@RequestScoped
public class QueueSongResource {

    @Inject
    SongMapper songMapper;
    @Inject
    QueueUserService queueUserService;
    @Inject
    QueueService queueService;

    QueuedSong song = null;
    QueueUser user = null;

    public QueueSongResource of(QueuedSong song, QueueUser user) {
        this.song = song;
        this.user = user;
        return this;
    }

    @PermitAll
    void onBeginTransaction(@Observes @Initialized(TransactionScoped.class) Object event) {
        if (song != null)
            song = QueuedSong.getEntityManager().merge(song);
        if (user != null)
            user = QueueUser.getEntityManager().merge(user);
    }

    @GET
    @Operation(summary = "Get the queued song", description = """
            Get the full state of the queued song, with all data.""")
    @APIResponse(responseCode = "200", description = "The queued song", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueuedSongQueryDto.class))
    })
    public QueuedSongQueryDto get() {
        return songMapper.toDto(song, queueUserService.likes(user, song));
    }

    @POST
    @Path("/play")
    @Operation(summary = "Play this song", description = """
            Play this song, ignoring the order of the queue.""")
    @Transactional
    public void like() throws ActionNotPermittedException {
        queueService.next(song);
    }

    @POST
    @Path("/likes")
    @Operation(summary = "Add a like to the song", description = """
            Add a like to the song, pushing it upwards in the queue.

            If the maximum number of likes was already reached, the oldest like will be removed.
            This will happen trasparently even if a number of likes larger than available is specified,
            effectively removing all likes and moving them to the song.""")
    @Parameter(name = "count", description = "How many time to like the song. If negative, nothing will happen.")
    @Transactional
    public void like(@QueryParam("count") @DefaultValue("1") short count)
            throws ActionNotPermittedException {
        queueService.like(song, user, count);
    }

    @DELETE
    @Operation(summary = "Remove this song from the queue", description = """
            Remove the song from the queue.

            TODO: ban functionality.""")
    @Transactional
    public void delete() throws ActionNotPermittedException {
        queueService.removeQueuedSong(song, user);
    }

}
