package io.github.zannabianca1997.apelle.queues.utils;

import java.util.UUID;

import org.jboss.logging.Logger;

import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.smallrye.mutiny.Multi;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.eventbus.EventBus;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.transaction.RollbackException;
import jakarta.transaction.Status;
import jakarta.transaction.Synchronization;
import jakarta.transaction.SystemException;
import jakarta.transaction.TransactionManager;

/**
 * Typed event bus
 * 
 * Publish events and generate listeners
 */
@ApplicationScoped
public class QueueEventBus {
    final Logger log;
    final EventBus eventBus;
    final TransactionManager transactionManager;

    public QueueEventBus(
            final Logger log,
            final EventBus eventBus,
            final TransactionManager transactionManager) {
        this.log = log;
        this.eventBus = eventBus;
        this.transactionManager = transactionManager;
    }

    // Use the full qualified class name to avoid collisions with other event buses
    final static String ADDRESS_SUFFIX = "@" + QueueEventBus.class.getName();

    /**
     * Calculate the address to send a queue event to
     * 
     * @param queueId The id of the target queue
     * @return The calculated address
     */
    private static String address(final UUID queueId) {
        final var builder = new StringBuilder(36 + ADDRESS_SUFFIX.length());
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
    public void publish(final QueueEvent event) {
        // Need to handle the transaction.
        //
        // As the events show the queue state, they need to be run AFTER the state has
        // changed. This means that if a transaction is ongoing, the message need to be
        // delayed until the transaction has completed
        try {
            if (transactionManager.getStatus() == Status.STATUS_NO_TRANSACTION) {
                // Direct publish
                doPublish(event);
            } else {
                // Need to run the publish AFTER the transaction commits
                transactionManager.getTransaction().registerSynchronization(new Synchronization() {
                    @Override
                    public void beforeCompletion() {
                        // nothing here
                    }

                    @Override
                    public void afterCompletion(final int status) {
                        doPublish(event);
                    }
                });
            }
        } catch (SystemException | IllegalStateException | RollbackException e) {
            throw new RuntimeException("Error while publishing event %s".formatted(event), e);
        }
    }

    private void doPublish(final QueueEvent event) {
        log.debugf("[queue=%s] Publishing event %s", event.getQueueId(), event.getClass());
        eventBus.publish(address(event.getQueueId()), JsonObject.mapFrom(event));
    }

    /**
     * Receive the queue events
     * 
     * @param queueId The queue to listen for
     * @return A stream of events
     */
    public Multi<QueueEvent> events(final UUID queueId) {
        return eventBus.<JsonObject>consumer(address(queueId))
                .toMulti().map(message -> message.body().mapTo(QueueEvent.class));
    }
}