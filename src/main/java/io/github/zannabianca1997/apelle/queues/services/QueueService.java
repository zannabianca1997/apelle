package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.vertx.mutiny.ext.web.Session;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.persistence.EntityManager;
import lombok.NonNull;

@ApplicationScoped
public class QueueService {

    @Inject
    EntityManager entityManager;

    @Inject
    QueueMapper queueMapper;

    /**
     * Handler for a running queue
     */
    public class QueueHandler {
        private Queue queue;

        private record UserSession(
                ApelleUser user, Session session) {
        }

        private ConcurrentHashMap<UUID, UserSession> sessions;

        public QueueHandler(@NonNull Queue queue) {
            this.queue = queue;

            sessions = new ConcurrentHashMap<>();
        }

        /**
         * Start the music
         * 
         * @throws CantPlayEmptyQueue Cannot play a empty queue
         */
        public boolean play() throws CantPlayEmptyQueue {
            var stopped = queue.play();
            // TODO: signal all sessions
            return stopped;
        }

        /**
         * Stops the music
         */
        public boolean stop() {
            var playing = queue.stop();
            // TODO: signal all sessions
            return playing;
        }

        /**
         * Close this handler, stopping the queue and closing all sessions
         */
        public void close() {
            stop();
            // TODO: signal all sessions
        }

        /**
         * Add a song to the queue
         * 
         * @param song The song to add
         * @return The added song
         */
        public QueuedSong enqueue(Song song) {
            var enqueued = queue.enqueue(song);
            // TODO: signal all sessions
            return enqueued;
        }

        /**
         * Activate this handler, adding the entity to the persistence context
         */
        public void activate() {
            queue = entityManager.merge(queue);
        }
    }

    /**
     * All the currently running handlers
     */
    private ConcurrentMap<UUID, QueueHandler> handlers = new ConcurrentHashMap<>();

    /**
     * Stop all sessions, persist everything to the database
     */
    void destroy() {
        // Stop all sessions
        for (QueueHandler handler : handlers.values()) {
            handler.activate();
            handler.close();
        }
        handlers.clear();
    }

    /**
     * Start the music
     * 
     * @throws CantPlayEmptyQueue     Cannot play a empty queue
     * @throws QueueNotFoundException The queue does not exist
     */
    public boolean play(@NonNull UUID queueId) throws CantPlayEmptyQueue, QueueNotFoundException {
        return getHandler(queueId).play();
    }

    /**
     * Stops the music
     * 
     * @throws QueueNotFoundException The queue does not exist
     */
    public boolean stop(@NonNull UUID queueId) throws QueueNotFoundException {
        return getHandler(queueId).stop();
    }

    /**
     * Add a song to a queue
     * 
     * @param queueId The id of the queue
     * @param song    The song to add
     * @return The added song
     * @throws QueueNotFoundException No queue with that id
     */
    public QueuedSong enqueue(@NonNull UUID queueId, @NonNull Song song) throws QueueNotFoundException {
        return getHandler(queueId).enqueue(song);
    }

    /**
     * Get or create the handler for the given queue
     * 
     * @param queueId The id of the queue
     * @return The handler
     * @throws QueueNotFoundException The queue does not exist
     */
    private QueueHandler getHandler(@NonNull UUID queueId) throws QueueNotFoundException {
        var handler = handlers.compute(queueId, (id, presentHandler) -> {
            if (presentHandler != null) {
                presentHandler.activate();
                return presentHandler;
            }
            var queue = (Queue) Queue.findById(id);
            if (queue == null) {
                return null;
            }
            return new QueueHandler(queue);
        });
        if (handler == null) {
            throw new QueueNotFoundException(queueId);
        }
        return handler;
    }
}