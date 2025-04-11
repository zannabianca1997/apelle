package io.github.zannabianca1997.apelle.queues.utils;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.smallrye.mutiny.Multi;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.eventbus.EventBus;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

/**
 * Typed event bus
 * 
 * Publish events and generate listeners
 */
@ApplicationScoped
public class QueueEventBus {

    @Inject
    EventBus eventBus;

    // Use the full qualified class name to avoid collisions with other event buses
    final static String ADDRESS_SUFFIX = "@" + QueueEventBus.class.getName();

    /**
     * Calculate the address to send a queue event to
     * 
     * @param queueId The id of the target queue
     * @return The calculated address
     */
    private static String address(UUID queueId) {
        var builder = new StringBuilder(36 + ADDRESS_SUFFIX.length());
        builder.append(queueId);
        builder.append(ADDRESS_SUFFIX);
        return builder.toString();
    }

    /**
     * Send a queue event.
     * 
     * The event is published on the address equal to the queue ID.
     * 
     * @param event The event to publish
     */
    public void publish(QueueEvent event) {
        eventBus.publish(address(event.getQueueId()), JsonObject.mapFrom(event));
    }

    /**
     * Receive the queue events
     * 
     * @param queue The queue to listen for
     * @return A stream of events
     */
    public Multi<QueueEvent> events(Queue queue) {
        return eventBus.<JsonObject>consumer(address(queue.getId()))
                .toMulti().map(message -> message.body().mapTo(QueueEvent.class));
    }
}