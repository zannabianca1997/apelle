package io.github.zannabianca1997.apelle.queues.resources;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.dtos.websocket.server.ServerMessage;
import io.github.zannabianca1997.apelle.queues.dtos.websocket.server.UnknowQueueMessage;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.mappers.EventMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.quarkus.security.Authenticated;
import io.quarkus.websockets.next.OnOpen;
import io.quarkus.websockets.next.PathParam;
import io.quarkus.websockets.next.WebSocket;
import io.quarkus.websockets.next.WebSocketConnection;
import io.smallrye.mutiny.Multi;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.eventbus.EventBus;
import jakarta.inject.Inject;

@WebSocket(path = "/ws/v1/queues/i/{queueId}")
@Authenticated
public class QueueWebsocket {
    @Inject
    EventBus eventBus;
    @Inject
    EventMapper eventMapper;

    @OnOpen
    Multi<ServerMessage> open(
            @PathParam String queueId, WebSocketConnection connection) {
        // Check that the id is a queue id
        UUID uuid;
        try {
            uuid = UUID.fromString(queueId);
        } catch (IllegalArgumentException e) {
            uuid = null;
        }

        if (uuid == null || Queue.findById(uuid) == null) {
            return Multi.createFrom()
                    .item((ServerMessage) UnknowQueueMessage.builder().queueId(connection.pathParam("queueId")).build())
                    .onCompletion().call(() -> connection.close());
        }

        // Create a listener on the uuid address, and handle all events from it
        return eventBus
                .<JsonObject>consumer(uuid.toString())
                .toMulti()
                .map(jsonObject -> jsonObject.body().mapTo(QueueEvent.class))
                .map(eventMapper::toMessage);
    }
}
