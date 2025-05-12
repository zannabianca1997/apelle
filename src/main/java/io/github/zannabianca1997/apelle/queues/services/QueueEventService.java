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

@ApplicationScoped
public class QueueEventService {
    private final QueueUserService queueUserService;

    public QueueEventService(final QueueUserService queueUserService) {
        this.queueUserService = queueUserService;
    }

    /**
     * Fill an event with user specific data, like likes given, etc
     */
    public QueueEvent asSeenBy(final QueueEvent event, final UUID userId) throws InvalidEvent {
        try {
            return switch (event) {
                case final QueueEnqueueEvent queueEnqueueEvent -> fillUserLikes(queueEnqueueEvent, userId);
                case final QueueLikeEvent queueLikeEvent -> fillUserLikes(queueLikeEvent, userId);
                case final QueueNextEvent queueNextEvent -> fillUserLikes(queueNextEvent, userId);
                case final QueueStartEvent queueStartEvent -> fillUserLikes(queueStartEvent, userId);
                default -> event;
            };
        } catch (SongNotQueuedException | QueueNotFoundException e) {
            throw new InvalidEvent("An event was in a invalid state", e);
        }
    }

    private QueueEnqueueEvent fillUserLikes(final QueueEnqueueEvent queueEnqueueEvent, final UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueEnqueueEvent.getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueEnqueueEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueEnqueueEvent;
    }

    private QueueLikeEvent fillUserLikes(final QueueLikeEvent queueLikeEvent, final UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueLikeEvent.getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueLikeEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueLikeEvent;
    }

    private QueueNextEvent fillUserLikes(final QueueNextEvent queueNextEvent, final UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueNextEvent.getState().getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueNextEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueNextEvent;
    }

    private QueueStartEvent fillUserLikes(final QueueStartEvent queueStartEvent, final UUID userId)
            throws SongNotQueuedException, QueueNotFoundException {
        for (final var queuedSongShortQueryDto : queueStartEvent.getState().getQueuedSongs()) {
            queuedSongShortQueryDto.setUserLikes(
                    queueUserService.likes(userId, queueStartEvent.getQueueId(), queuedSongShortQueryDto.getId()));
        }
        return queueStartEvent;
    }

}
