package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.exceptions.SongNotQueuedException;
import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import jakarta.enterprise.context.ApplicationScoped;

@ApplicationScoped
public class QueueUserService {

    private final QueueService queueService;
    private final UsersService usersService;

    public QueueUserService(final QueueService queueService, final UsersService usersService) {
        this.queueService = queueService;
        this.usersService = usersService;
    }

    /**
     * Get the queue user for the current user
     * 
     * @param queueId The queue id
     * @return The queue user
     */
    public QueueUser getCurrent(final Queue queue) {
        return findOrCreate(queue, usersService.getMe());
    }

    /**
     * Get a queue user by name
     * 
     * @param queueId  The queue id
     * @param userName The user name
     * @return The queue user
     * @throws UserNotFoundByNameException The user does not exist
     */
    public QueueUser getByName(final Queue queue, final String userName)
            throws UserNotFoundByNameException {
        return findOrCreate(queue, usersService.getByName(userName));
    }

    /**
     * Get a queue user by name
     * 
     * @param queueId The queue id
     * @param userId  The user id
     * @return The queue user
     * @throws UserNotFoundByIdException The user does not exist
     */
    public QueueUser getById(final Queue queue, final UUID userId) throws UserNotFoundByIdException {
        return findOrCreate(queue, usersService.getById(userId));
    }

    /**
     * Find the queue user, or create a new one.
     * 
     * The newly created queue user is not persisted automatically,
     * as it has the default role.
     * 
     * @param queueId The queue id
     * @param user    The user to link
     * @return The found or created queue user
     */
    private QueueUser findOrCreate(final Queue queue, final ApelleUser user) {
        final QueueUser queueUser = QueueUser.findById(user, queue);
        if (queueUser == null) {
            return QueueUser.builder()
                    .queue(queue)
                    .user(user)
                    .role(queue.getConfig().getDefaultRole())
                    .likesFilled(true)
                    .build();
        }
        return queueUser;
    }

    public void delete(final QueueUser user) throws ActionNotPermittedException {
        final QueueUser deleter = getCurrent(user.getQueue());
        if (!deleter.getPermissions().getQueueUsers().isRemove()
                && deleter.getUser().getId() != user.getUser().getId()) {
            throw new ActionNotPermittedException(deleter.getRole(), "remove user");
        }
        user.delete();
    }

    public short likes(final QueueUser user, final QueuedSong song) {
        assert user.getQueue().getId().equals(song.getQueue().getId());
        return likes(user.getUser(), song);
    }

    public short likes(final ApelleUser user, final QueuedSong song) {
        return likes(user.getId(), song);
    }

    public short likes(final UUID userId, final QueuedSong song) {
        return Likes.givenBy(userId, song);
    }

    public short likes(final UUID userId, final UUID queueId, final UUID songId)
            throws SongNotQueuedException, QueueNotFoundException {
        return Likes.givenBy(userId, queueService.getQueuedSong(queueService.get(queueId), songId));
    }
}
