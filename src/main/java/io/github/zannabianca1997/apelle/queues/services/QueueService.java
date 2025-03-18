package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;
import java.util.concurrent.ConcurrentHashMap;
import java.util.concurrent.ConcurrentMap;

import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.vertx.mutiny.ext.web.Session;
import jakarta.annotation.PreDestroy;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.transaction.Transactional;
import lombok.NonNull;

@ApplicationScoped
public class QueueService {

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
        public void play() throws CantPlayEmptyQueue {
            queue.play();
            // TODO: signal all sessions
        }

        /**
         * Stops the music
         */
        public void stop() {
            queue.stop();
            // TODO: signal all sessions
        }

        /**
         * Close this queue
         */
        public void close() {
            stop();
            // TODO: signal all sessions
        }

        public QueuedSong enqueue(Song song) {
            var enqueued = queue.enqueue(song);
            // TODO: signal all sessions
            return enqueued;
        }
    }

    /**
     * All the currently running handlers
     */
    private ConcurrentMap<UUID, QueueHandler> handlers = new ConcurrentHashMap<>();

    @PreDestroy
    @Transactional
    void destroy() {
        // Stop all sessions
        for (QueueHandler handler : handlers.values()) {
            handler.close();
        }
        handlers.clear();
    }

    /**
     * Add a song to a queueu
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
        var handler = handlers.computeIfAbsent(queueId, id -> {
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