package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueuePlayEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.security.identity.SecurityIdentity;
import io.smallrye.mutiny.Uni;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.eventbus.EventBus;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class QueueService {

    @Inject
    EventBus eventBus;
    @Inject
    QueueMapper queueMapper;
    @Inject
    SongService songService;
    @Inject
    QueueUserService queueUserService;

    @Inject
    SecurityIdentity securityIdentity;

    /**
     * Create a new queue
     * 
     * @return The created queue
     */
    public Queue create() {
        ApelleUser currentUser = ApelleUser.findByName(securityIdentity.getPrincipal().getName());

        var queue = Queue.builder()
                .admin(currentUser)
                .build();

        queue.persist();
        return queue;
    }

    /**
     * Obtain a queue
     * 
     * @param queueId The id of the queue
     * @return The found queue
     * @throws QueueNotFoundException The queue does not exist
     */
    public Queue get(UUID queueId) throws QueueNotFoundException {
        Queue queue = Queue.findById(queueId);
        if (queue == null) {
            throw new QueueNotFoundException(queueId);
        }
        return queue;
    }

    /**
     * Start playing a queue
     * 
     * @param queueId The id of the queue
     * @throws QueueNotFoundException The queue does not exist
     * @throws CantPlayEmptyQueue     The queue is empty
     */
    public void play(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        Queue queue = get(queueId);
        boolean startedNow = queue.play();
        if (startedNow) {
            publish(QueuePlayEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());
            scheduleStopAtEnd(queue);
        }
    }

    /**
     * Stop a playing queue
     * 
     * @param queueId The id on the queue
     * @throws QueueNotFoundException The queue does not exist
     */
    public void stop(UUID queueId)
            throws QueueNotFoundException {
        Queue queue = get(queueId);
        boolean stoppedNow = queue.stop();
        if (stoppedNow) {
            publish(QueueStopEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());
        }
    }

    /**
     * Skip to the next song
     * 
     * @param queueId The id on the queue
     * @throws QueueNotFoundException The queue does not exist
     * @throws CantPlayEmptyQueue     The queue is empty
     */
    public void next(UUID queueId)
            throws QueueNotFoundException, CantPlayEmptyQueue {
        Queue queue = get(queueId);
        queue.next();
        publish(QueueNextEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());
        scheduleStopAtEnd(queue);
    }

    /**
     * Add a song to the queueu
     * 
     * @param queueId The id on the queue
     * @param song    The song to add
     * @return The queued song
     * @throws QueueNotFoundException The queue does not exist
     */
    public QueuedSong enqueue(UUID queueId, Song song) throws QueueNotFoundException {
        Queue queue = get(queueId);
        QueuedSong enqueued = queue.enqueue(song);
        enqueued.persist();
        publish(QueueEnqueueEvent.builder().queueId(queueId).state(queueMapper.toDto(queue)).build());
        return enqueued;
    }

    /**
     * Send a queue event.
     * 
     * The event is published on the address equal to the queue ID.
     * 
     * @param event The event to publish
     */
    private void publish(QueueEvent event) {
        eventBus.publish(event.getQueueId().toString(), JsonObject.mapFrom(event));
    }

    /**
     * Schedule the queue to be stopped when it finished
     * 
     * @param queue The queue to stop
     */
    private void scheduleStopAtEnd(Queue queue) {
        final UUID queueId = queue.getId();
        // Fire when the song would end
        Uni<Boolean> songEnded = Uni.createFrom().voidItem()
                .onItem().delayIt().by(queue.getCurrent().timeLeft())
                .replaceWith(false);
        // Fire if something stop the song
        Uni<Boolean> stopEvent = eventBus.<JsonObject>consumer(queueId.toString())
                .toMulti()
                .map(jsonObject -> jsonObject.body().mapTo(QueueEvent.class))
                .filter(event -> event instanceof QueueStopEvent)
                .onItem().castTo(QueueStopEvent.class)
                .toUni().replaceWith(true);
        // On song completion, if nothing stopped it before, stop the song
        Uni.combine().any().of(songEnded, stopEvent)
                .subscribe().with(stopped -> {
                    if (!stopped) {
                        try {
                            stop(queueId);
                        } catch (QueueNotFoundException e) {
                            // Queue was deleted, nothing to do
                        }
                    }
                });
    }

}
