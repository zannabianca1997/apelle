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
import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.exceptions.CantPlayEmptyQueueException;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongAlreadyQueuedException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueuedException;
import io.github.zannabianca1997.apelle.queues.mappers.QueueMapper;
import io.github.zannabianca1997.apelle.queues.mappers.SongMapper;
import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.queues.models.Song;
import io.github.zannabianca1997.apelle.queues.utils.QueueEventBus;
import io.github.zannabianca1997.apelle.queues.utils.StringUtils;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import io.smallrye.mutiny.Multi;
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
    QueueUserRolesService queueUserRolesService;
    @Inject
    QueueUserService queueUserService;

    @Inject
    QueueCodeConfigs queueCodeConfigs;

    @Inject
    QueueEventBus queueEventBus;

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
     * @throws QueueNotFoundException      The queue does not exist
     * @throws CantPlayEmptyQueueException The queue is empty
     * @throws ActionNotPermittedException
     */
    public void start(Queue queue) throws CantPlayEmptyQueueException, ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().queue().start()) {
            throw new ActionNotPermittedException(user.getRole(), "start playing");
        }

        boolean startedNow = queue.start();
        if (startedNow) {
            queueEventBus
                    .publish(QueueStartEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
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
        if (!user.getPermissions().queue().stop()) {
            throw new ActionNotPermittedException(user.getRole(), "stop playing");
        }

        stopNoCheck(queue);
    }

    private void stopNoCheck(Queue queue) {
        boolean stoppedNow = queue.stop();
        if (stoppedNow) {
            queueEventBus
                    .publish(QueueStopEvent.builder().queueId(queue.getId())
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
        if (!user.getPermissions().queue().next()) {
            throw new ActionNotPermittedException(user.getRole(), "move to next song");
        }

        queue.next();
        queueEventBus.publish(QueueNextEvent.builder().queueId(queue.getId()).state(queueMapper.toDto(queue)).build());
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
    public QueuedSong enqueue(Queue queue, Song song) throws SongAlreadyQueuedException, ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().queue().enqueue()) {
            throw new ActionNotPermittedException(user.getRole(), "enqueue song");
        }

        if (queue.getAllSongs().anyMatch(queued -> queued.isSame(song))) {
            throw new SongAlreadyQueuedException(queue.getId(), song);
        }
        song.persist();

        QueuedSong enqueued = queue.enqueue(song);

        queueEventBus
                .publish(
                        QueueEnqueueEvent.builder().queueId(queue.getId())
                                .queuedSongs(queue.getQueuedSongs().stream().map(songMapper::toShortDto).toList())
                                .build());

        return enqueued;
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
        if (!user.getPermissions().queue().like()) {
            throw new ActionNotPermittedException(user.getRole(), "like song");
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
            QueuedSong removingFrom = queue.getQueuedSongs().stream()
                    .filter(song2 -> song2.getSong().getId().equals(oldests.getSong().getId()))
                    .findAny().orElseThrow();
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
                .queuedSongs(queue.getQueuedSongs().stream().map(songMapper::toShortDto).toList()).build());
    }

    public QueuedSong getQueuedSong(Queue queue, UUID songId) throws SongNotQueuedException {
        QueuedSong queuedSong = QueuedSong.findById(songId, queue);
        if (queuedSong == null) {
            throw new SongNotQueuedException(queue, songId);
        }
        return queuedSong;
    }

    public void delete(Queue queue) throws ActionNotPermittedException {
        QueueUser user = queueUserService.getCurrent(queue);
        if (!user.getPermissions().delete()) {
            throw new ActionNotPermittedException(user.getRole(), "delete queue");
        }

        queue.delete();

        // Annunce the queue was deleted
        queueEventBus.publish(QueueDeleteEvent.builder().queueId(queue.getId()).build());
    }

    public Multi<QueueEvent> events(Queue queue) {
        return queueEventBus.events(queue);
    }

}
