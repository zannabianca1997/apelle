package io.github.zannabianca1997.apelle.queues.utils;

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

    /**
     * Send a queue event.
     * 
     * The event is published on the address equal to the queue ID.
     * 
     * @param event The event to publish
     */
    public void publish(QueueEvent event) {
        eventBus.publish(event.getQueueId().toString(), JsonObject.mapFrom(event));
    }

    /**
     * Receive the queue events
     * 
     * @param queue The queue to listen for
     * @return A stream of events
     */
    public Multi<QueueEvent> events(Queue queue) {
        return eventBus.<JsonObject>consumer(queue.getId().toString())
                .toMulti().map(message -> message.body().mapTo(QueueEvent.class));
    }
}