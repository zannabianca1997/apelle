package io.github.zannabianca1997.apelle.queues.services;

import java.time.Instant;
import java.util.UUID;

import org.jboss.logging.Logger;

import io.github.zannabianca1997.apelle.queues.configs.QueueCodeConfigs;
import io.github.zannabianca1997.apelle.queues.events.QueueDeleteEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueLikeEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStartEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStopEvent;
import io.github.zannabianca1997.apelle.queues.events.QueuedSongDeleteEvent;
import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueueException;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongAlreadyQueuedException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueuedException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueConfig;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.utils.QueueEventBus;
import io.github.zannabianca1997.apelle.queues.utils.StringUtils;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.smallrye.mutiny.Multi;
import io.smallrye.mutiny.infrastructure.Infrastructure;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class QueueService {
    @Inject
    QueueMapper queueMapper;
    @Inject
    SongMapper songMapper;

    @Inject
    UsersService usersService;
    @Inject
    QueueUserService queueUserService;
    @Inject
    QueueEventService queueEventService;

    @Inject
    QueueCodeConfigs queueCodeConfigs;

    @Inject
    QueueEventBus queueEventBus;

    @Inject
    StringUtils stringUtils;

    @Inject
    Logger log;

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
                .config(QueueConfig.findDefault())
                .build();
        var creator = usersService.getMe();
        queue.getUsers().add(QueueUser.builder()
                .queue(queue)
                .user(creator)
                .role(queue.getConfig().getCreatorRole())
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

        log.infof("[user=%s, queue=%s] Created queue", creator.getId(), queue.getId());

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
     * @throws QueueNotFoundException      The queue does not exist
     * @throws CantPlayEmptyQueueException The queue is empty
     * @throws ActionNotPermittedException
     */
    public void start(Queue queue) throws CantPlayEmptyQueueException, ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().getQueue().isStart()) {
            throw new ActionNotPermittedException(user.getRole(), "start playing");
        }

        log.infof("[user=%s, queue=%s] Start playing requested", user.getUser().getId(), queue.getId());

        boolean startedNow = queue.start();
        if (startedNow) {
            queueEventBus
                    .publish(QueueStartEvent.builder().queueId(queue.getId())
                            .state(queueMapper.toDto(queue, song -> (short) -1)).build());
        }
    }

    /**
     * Stop a playing queue
     * 
     * @param queueId The id on the queue
     * @throws ActionNotPermittedException
     * @throws QueueNotFoundException      The queue does not exist
     */
    public void stop(Queue queue) throws ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().getQueue().isStop()) {
            throw new ActionNotPermittedException(user.getRole(), "stop playing");
        }

        log.infof("[user=%s, queue=%s] Stop playing requested", user.getUser().getId(), queue.getId());

        boolean stoppedNow = queue.stop();
        if (stoppedNow) {
            queueEventBus
                    .publish(QueueStopEvent.builder().queueId(queue.getId())
                            .playerStateId(queue.getPlayerStateId())
                            .state(songMapper.toDto(queue.getCurrent())).build());
        }
    }

    /**
     * Skip to the next song
     * 
     * @param queueId The id on the queue
     * @throws QueueNotFoundException      The queue does not exist
     * @throws CantPlayEmptyQueueException The queue is empty
     * @throws ActionNotPermittedException
     */
    public void next(Queue queue) throws CantPlayEmptyQueueException, ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().getQueue().isNext()) {
            throw new ActionNotPermittedException(user.getRole(), "move to next song");
        }

        log.infof("[user=%s, queue=%s] Next song requested", user.getUser().getId(), queue.getId());

        queue.next();
        queueEventBus.publish(QueueNextEvent.builder().queueId(queue.getId())
                .state(queueMapper.toDto(queue, song -> (short) -1)).build());
    }

    /**
     * Skip to a given song
     * 
     * @param song The song to skip to
     * @throws ActionNotPermittedException The user can't skip songs
     */
    public void next(QueuedSong song) throws ActionNotPermittedException {
        Queue queue = song.getQueue();

        QueueUser user = queueUserService.getCurrent(song.getQueue());
        if (!user.getPermissions().getQueue().isNext()) {
            throw new ActionNotPermittedException(user.getRole(), "move to song");
        }

        log.infof("[user=%s, queue=%s] Song %s requested", user.getUser().getId(), queue.getId(),
                song.getSong().getId());

        queue.next(song);
        queueEventBus.publish(QueueNextEvent.builder().queueId(queue.getId())
                .state(queueMapper.toDto(queue, s -> (short) -1)).build());
    }

    public record EnqueueResult(QueuedSong queuedSong, short autolikes) {
    }

    /**
     * Add a song to the queueu
     * 
     * @param queueId The id on the queue
     * @param song    The song to add
     * @return The queued song
     * @throws SongAlreadyQueuedException  The song is already in the queue
     * @throws ActionNotPermittedException
     */
    public EnqueueResult enqueue(Queue queue, Song song, Boolean autolikeOverride)
            throws SongAlreadyQueuedException, ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().getQueue().isEnqueue()) {
            throw new ActionNotPermittedException(user.getRole(), "enqueue song");
        }

        if (queue.getAllSongs().anyMatch(queued -> queued.isSame(song))) {
            throw new SongAlreadyQueuedException(queue.getId(), song);
        }
        song.persist();

        log.infof("[user=%s, queue=%s] Song added: %s", user.getUser().getId(), queue.getId(), song.getId());

        QueuedSong enqueued = queue.enqueue(song);

        // Calculate autolike
        final boolean autolike = autolikeOverride != null ? autolikeOverride : queue.getConfig().isAutolike();
        final short likesGiven;
        if (autolike && user.getAvailableLikes() > 0) {
            Likes.builder().user(user.getUser()).song(enqueued).givenAt(Instant.now()).count((short) 1).build()
                    .persist();
            enqueued.setLikes((short) 1);
            queue.sortSongs();
            likesGiven = (short) 1;
        } else {
            likesGiven = (short) 0;
        }

        queueEventBus
                .publish(
                        QueueEnqueueEvent.builder().queueId(queue.getId())
                                .queuedSongs(queue.getQueuedSongs().stream()
                                        .map(s -> songMapper.toShortDto(s, (short) -1))
                                        .toList())
                                .build());

        return new EnqueueResult(enqueued, likesGiven);
    }

    /**
     * Add a like to a song
     * 
     * @param song   The song to like
     * @param userId The user liking the song
     * @throws ActionNotPermittedException
     */
    public void like(QueuedSong song, QueueUser user) throws ActionNotPermittedException {
        like(song, user, (short) 1);
    }

    /**
     * Add many like to a song
     * 
     * @param song   The song to like
     * @param userId The user liking the song
     * @param count  The number of like to add
     * @throws ActionNotPermittedException
     */
    public void like(QueuedSong song, QueueUser user, short count) throws ActionNotPermittedException {
        if (!user.getPermissions().getQueue().isLike()) {
            throw new ActionNotPermittedException(user.getRole(), "like song");
        }

        if (count < 1) {
            // Nothing to do
            return;
        }

        Queue queue = song.getQueue();

        log.infof("[user=%s, queue=%s] User adds %s likes to the song %s", user.getUser().getId(), queue.getId(), count,
                song.getSong().getId());

        // Liming the number of likes to the max
        count = (short) Math.min(count, user.getMaxLikes());

        // Calculating how many likes must be removed
        int toRemove = Math.max(count - (user.getMaxLikes() - user.getLikes()), 0);
        while (toRemove > 0) {
            // Find the oldest group of likes
            Likes oldests = Likes.findOldests(user);

            // calculate how many to remove
            var removing = Math.min(toRemove, oldests.getCount());
            oldests.setCount((short) (oldests.getCount() - removing));
            toRemove -= removing;

            // Mark row for deletion id all the likes where removed
            if (oldests.getCount() == 0) {
                oldests.delete();
            }

            // Remove likes from the queue in memory
            QueuedSong removingFrom = oldests.getSong();
            removingFrom.setLikes((short) (removingFrom.getLikes() - removing));
        }

        Instant now = Instant.now();
        Likes likes = Likes.findById(user.getUser(), song, now);

        if (likes == null) {
            likes = Likes.builder().user(user.getUser()).song(song).givenAt(now).count(count).build();
            likes.persist();
        } else {
            likes.setCount((short) (likes.getCount() + count));
        }

        // Adding likes to the queue in memory
        QueuedSong addingTo = queue.getQueuedSongs().stream()
                .filter(song2 -> song2.getSong().getId().equals(song.getSong().getId()))
                .findAny().orElseThrow();
        addingTo.setLikes((short) (addingTo.getLikes() + count));

        // Sorting the song, as likes have changed
        queue.sortSongs();

        // Signal songs have changed
        queueEventBus.publish(QueueLikeEvent.builder().queueId(queue.getId())
                .queuedSongs(queue.getQueuedSongs().stream()
                        .map(s -> songMapper.toShortDto(s, (short) -1)).toList())
                .build());
    }

    public QueuedSong getQueuedSong(Queue queue, UUID songId) throws SongNotQueuedException {
        QueuedSong queuedSong = QueuedSong.findById(songId, queue);
        if (queuedSong == null) {
            throw new SongNotQueuedException(queue, songId);
        }
        return queuedSong;
    }

    public void removeQueuedSong(QueuedSong song, QueueUser user) throws ActionNotPermittedException {
        if (!user.getPermissions().getQueue().isRemove()) {
            throw new ActionNotPermittedException(user.getRole(), "remove song");
        }

        log.infof("[user=%s, queue=%s] Removed song %s", user.getUser().getId(), user.getQueue().getId(),
                song.getSong().getId());

        Likes.deleteReferringTo(song);
        song.getQueue().getQueuedSongs().removeIf(s -> s.getSong().getId().equals(song.getSong().getId()));
        song.getSong().getQueues().removeIf(s -> s.getQueue().getId().equals(song.getQueue().getId()));
        song.delete();

        queueEventBus.publish(QueuedSongDeleteEvent.builder().queueId(song.getQueue().getId())
                .deletedId(song.getSong().getId()).build());
    }

    public void delete(Queue queue) throws ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().isDelete()) {
            throw new ActionNotPermittedException(user.getRole(), "delete queue");
        }

        log.infof("[user=%s, queue=%s] Queue deleted", user.getUser().getId(), queue.getId());

        queue.delete();

        // Annunce the queue was deleted
        queueEventBus.publish(QueueDeleteEvent.builder().queueId(queue.getId()).build());
    }

    public Multi<QueueEvent> events(Queue queue, QueueUser seenBy) {
        // Extract the ids, ensuring the entities are not captured by the multi and
        // persist for the entire request
        UUID queueId = queue.getId();
        UUID userId = seenBy.getUser().getId();

        return queueEventBus.events(queueId)
                // The `asSeenBy` call need to contact the db, so it must run on the worker pool
                .emitOn(Infrastructure.getDefaultWorkerPool())
                .map(event -> queueEventService.asSeenBy(event, userId))
                .onSubscription()
                .invoke(() -> log.infof("[user=%s, queue=%s] Connected to the server", userId, queueId))
                .onItem()
                .invoke(event -> log.debugf("[user=%s, queue=%s] Received event: %s", userId, queueId,
                        event.getClass()))
                .onCancellation()
                .invoke(() -> log.infof("[user=%s, queue=%s] Disconnetted from the server", userId, queueId));
    }

}
