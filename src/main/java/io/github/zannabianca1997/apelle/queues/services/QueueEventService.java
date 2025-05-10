package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.events.QueueEnqueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueLikeEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueNextEvent;
import io.github.zannabianca1997.apelle.queues.events.QueueStartEvent;
import io.github.zannabianca1997.apelle.queues.exceptions.InvalidEvent;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueuedException;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class QueueEventService {

    @Inject
    QueueUserService queueUserService;

    /**
     * Fill an event with user specific data, like likes given, etc
     */
    public QueueEvent asSeenBy(QueueEvent event, UUID userId) throws InvalidEvent {
        try {
            return switch (event) {
                case QueueEnqueueEvent queueEnqueueEvent -> fillUserLikes(queueEnqueueEvent, userId);
                case QueueLikeEvent queueLikeEvent -> fillUserLikes(queueLikeEvent, userId);
                case QueueNextEvent queueNextEvent -> fillUserLikes(queueNextEvent, userId);
                case QueueStartEvent queueStartEvent -> fillUserLikes(queueStartEvent, userId);
                default -> event;
            };
        } catch (SongNotQueuedException | QueueNotFoundException e) {
            throw new InvalidEvent("An event was in a invalid state", e);
        }
    }

    private QueueEnqueueEvent fillUserLikes(QueueEnqueueEvent queueEnqueueEvent, UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueEnqueueEvent.getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueEnqueueEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueEnqueueEvent;
    }

    private QueueLikeEvent fillUserLikes(QueueLikeEvent queueLikeEvent, UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueLikeEvent.getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueLikeEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueLikeEvent;
    }

    private QueueNextEvent fillUserLikes(QueueNextEvent queueNextEvent, UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueNextEvent.getState().getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueNextEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueNextEvent;
    }

    private QueueStartEvent fillUserLikes(QueueStartEvent queueStartEvent, UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueStartEvent.getState().getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueStartEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueStartEvent;
    }

}
