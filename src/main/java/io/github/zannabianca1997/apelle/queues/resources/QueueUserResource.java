package io.github.zannabianca1997.apelle.queues.resources;

import org.eclipse.microprofile.openapi.annotations.Operation;
import org.eclipse.microprofile.openapi.annotations.media.Content;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
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
import jakarta.ws.rs.GET;
import io.github.zannabianca1997.apelle.queues.dtos.QueueUserQueryDto;
import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueUserMapper;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.services.QueueUserService;

@Tag(name = "Queue Users", description = "Management of the queue users")
@Authenticated
@RequestScoped
public class QueueUserResource {

    @Inject
    QueueUserService queueUserService;
    @Inject
    QueueUserMapper queueUserMapper;

    QueueUser user = null;
    boolean isMe;

    public QueueUserResource of(QueueUser user) {
        this.user = user;
        this.isMe = false;
        return this;
    }

    public QueueUserResource ofMe(QueueUser user) {
        this.user = user;
        this.isMe = true;
        return this;
    }

    @PermitAll
    void onBeginTransaction(@Observes @Initialized(TransactionScoped.class) Object event) {
        if (user != null)
            user = QueueUser.getEntityManager().merge(user);
    }

    @GET
    @Operation(summary = "The queue user data", description = "Returns the data of the queue user")
    @APIResponse(responseCode = "200", description = "The queue user", content = {
            @Content(mediaType = "application/json", schema = @Schema(implementation = QueueUserQueryDto.class))
    })
    public QueueUserQueryDto get() {
        return queueUserMapper.toDto(user);
    }

    @DELETE
    @Transactional
    @Operation(summary = "Remove the user from the queue", description = "Remove the user from the queue. This will also remove all likes he has given.")
    @APIResponse(responseCode = "200", description = "The user was removed")
    public void delete() throws ActionNotPermittedException {
        queueUserService.delete(user);
    }
}
