package io.github.zannabianca1997.apelle.queues.services;

import java.time.Instant;
import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.configs.QueueCodeConfigs;
import io.github.zannabianca1997.apelle.queues.events.QueueDeleteEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueLikeEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStartEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;
import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermitted;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueue;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongAlreadyQueued;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueued;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.utils.StringUtils;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.smallrye.mutiny.Uni;
import io.vertx.core.json.JsonObject;
import io.vertx.mutiny.core.eventbus.EventBus;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class QueueService {
    @Inject
    QueueMapper queueMapper;

    @Inject
    UsersService usersService;
    @Inject
    QueueUserRolesService queueUserRolesService;
    @Inject
    QueueUserService queueUserService;

    @Inject
    EventBus eventBus;

    @Inject
    QueueCodeConfigs queueCodeConfigs;

    @Inject
    StringUtils stringUtils;

    /**
     * Create a new queue
     * 
     * @return The created queue
     */
    public Queue create() {

        /*
         * Evaluate needed code complexity.
         * 
         * A minimal lenght is required, then an estimate of how many bytes are needed
         * to avoid collisions is made. If all queue codes are assigned this will bring
         * to a logaritmical growth of the code lenght, keeping it under control.
         */
        var codeComplexity = Integer.max(queueCodeConfigs.complexity().min(),
                queueCodeConfigs.complexity().margin() + (int) (Math.log1p(Queue.count()) / Math.log(256)));

        var queue = Queue.builder()
                .code(generateQueueCode(codeComplexity))
                .build();
        queue.getUsers().add(QueueUser.builder()
                .queue(queue)
                .user(usersService.getMe())
                .role(queueUserRolesService.getCreatorRole())
                .likesFilled(false)
                .build());

        /*
         * Try to insert the queue.
         * 
         * In case of a code collision, generate a new code with an increase complexity
         * and try again until it succeed. As each try has about 1/256 more possibility
         * to suceed, this should very rarely exceed two tries.
         */
        while (Queue.existByCode(queue.getCode())) {
            codeComplexity += 1;
            queue.setCode(generateQueueCode(codeComplexity));
        }

        queue.persist();
        return queue;
    }

    private String generateQueueCode(int codeComplexity) {
        return stringUtils.random(codeComplexity, queueCodeConfigs.alphabet());
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
     * Obtain a queue by code
     * 
     * @param queueId The id of the queue
     * @return The found queue
     * @throws QueueNotFoundException The queue does not exist
     */
    public Queue get(String queueCode) throws QueueNotFoundException {
        Queue queue = Queue.findByCode(queueCode);
        if (queue == null) {
            throw new QueueNotFoundException(queueCode);
        }
        return queue;
    }

    /**
     * Start playing a queue
     * 
     * @param queueId The id of the queue
     * @throws QueueNotFoundException The queue does not exist
     * @throws CantPlayEmptyQueue     The queue is empty
     * @throws ActionNotPermitted
     */
    public void start(Queue queue) throws CantPlayEmptyQueue, ActionNotPermitted {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().queue().start()) {
            throw new ActionNotPermitted(user.getRole(), "start playing");
        }

        boolean startedNow = queue.start();
        if (startedNow) {
            publish(QueueStartEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
            scheduleStopAtEnd(queue);
        }
    }

    /**
     * Stop a playing queue
     * 
     * @param queueId The id on the queue
     * @throws ActionNotPermitted
     * @throws QueueNotFoundException The queue does not exist
     */
    public void stop(Queue queue) throws ActionNotPermitted {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().queue().stop()) {
            throw new ActionNotPermitted(user.getRole(), "stop playing");
        }

        stopNoCheck(queue);
    }

    private void stopNoCheck(Queue queue) {
        boolean stoppedNow = queue.stop();
        if (stoppedNow) {
            publish(QueueStopEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
        }
    }

    /**
     * Skip to the next song
     * 
     * @param queueId The id on the queue
     * @throws QueueNotFoundException The queue does not exist
     * @throws CantPlayEmptyQueue     The queue is empty
     * @throws ActionNotPermitted
     */
    public void next(Queue queue) throws CantPlayEmptyQueue, ActionNotPermitted {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().queue().next()) {
            throw new ActionNotPermitted(user.getRole(), "move to next song");
        }

        queue.next();
        publish(QueueNextEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
        scheduleStopAtEnd(queue);
    }

    /**
     * Add a song to the queueu
     * 
     * @param queueId The id on the queue
     * @param song    The song to add
     * @return The queued song
     * @throws SongAlreadyQueued  The song is already in the queue
     * @throws ActionNotPermitted
     */
    public QueuedSong enqueue(Queue queue, Song song) throws SongAlreadyQueued, ActionNotPermitted {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().queue().enqueue()) {
            throw new ActionNotPermitted(user.getRole(), "enqueue song");
        }

        if (queue.getAllSongs().anyMatch(queued -> queued.isSame(song))) {
            throw new SongAlreadyQueued(queue.getId(), song);
        }
        QueuedSong enqueued = queue.enqueue(song);
        enqueued.persist();
        publish(QueueEnqueueEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
        return enqueued;
    }

    /**
     * Add a like to a song
     * 
     * @param song   The song to like
     * @param userId The user liking the song
     * @throws ActionNotPermitted
     */
    public void like(QueuedSong song, QueueUser user) throws ActionNotPermitted {
        like(song, user, (short) 1);
    }

    /**
     * Add many like to a song
     * 
     * @param song   The song to like
     * @param userId The user liking the song
     * @param count  The number of like to add
     * @throws ActionNotPermitted
     */
    public void like(QueuedSong song, QueueUser user, short count) throws ActionNotPermitted {
        if (!user.getPermissions().queue().like()) {
            throw new ActionNotPermitted(user.getRole(), "like song");
        }

        if (count < 1) {
            // Nothing to do
            return;
        }

        Queue queue = song.getQueue();

        // Liming the number of likes to the max
        count = (short) Math.min(count, user.getMaxLikes());

        // Calculating how many likes must be removed
        int toRemove = Math.max(count - (user.getMaxLikes() - user.getLikes()), 0);
        while (toRemove > 0) {
            // Find the oldest group of likes
            Likes oldests = Likes.findOldests(user.getUser().getId(), song.getLink());

            // calculate how many to remove
            var removing = Math.min(toRemove, oldests.getCount());
            oldests.setCount((short) (oldests.getCount() - removing));
            toRemove -= removing;

            // Mark row for deletion id all the likes where removed
            if (oldests.getCount() == 0) {
                oldests.delete();
            }

            // Remove likes from the queue in memory
            QueuedSong removingFrom = queue.getQueuedSongs().stream()
                    .filter(song2 -> song2.getLink().getSong() == oldests.getLink().getSong())
                    .findAny().orElseThrow();
            removingFrom.setLikes((short) (removingFrom.getLikes() - removing));
        }

        Instant now = Instant.now();
        Likes likes = Likes.findById(user.getUser().getId(), song.getLink(), now);

        if (likes == null) {
            likes = Likes.builder().user(user.getUser()).song(song).givenAt(now).count(count).build();
            likes.persist();
        } else {
            likes.setCount((short) (likes.getCount() + count));
        }

        // Adding likes to the queue in memory
        QueuedSong addingTo = queue.getQueuedSongs().stream()
                .filter(song2 -> song2.getLink().getSong() == song.getLink().getSong())
                .findAny().orElseThrow();
        addingTo.setLikes((short) (addingTo.getLikes() + count));

        // Sorting the song, as likes have changed
        queue.sortSongs();

        // Signal songs have changed
        publish(QueueLikeEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
    }

    public QueuedSong getQueuedSong(Queue queue, UUID songId) throws SongNotQueued {
        QueuedSong queuedSong = QueuedSong.findById(songId, queue.getId());
        if (queuedSong == null) {
            throw new SongNotQueued(queue.getId(), songId);
        }
        return queuedSong;
    }

    public void delete(Queue queue) throws ActionNotPermitted {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().delete()) {
            throw new ActionNotPermitted(user.getRole(), "delete queue");
        }

        queue.delete();

        // Annunce the queue was deleted
        publish(QueueDeleteEvent.builder().queueId(queue.getId()).build());
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
                        stopNoCheck(queue);
                    }
                });
    }

}
