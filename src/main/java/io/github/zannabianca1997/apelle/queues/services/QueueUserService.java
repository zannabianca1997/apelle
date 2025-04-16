package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.exceptions.ActionNotPermittedException;
import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueuedSong;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.services.UsersService;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class QueueUserService {

    @Inject
    QueueService queueService;
    @Inject
    UsersService usersService;
    @Inject
    QueueUserRolesService queueUserRolesService;

    /**
     * Get the queue user for the current user
     * 
     * @param queueId The queue id
     * @return The queue user
     */
    public QueueUser getCurrent(Queue queue) {
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
    public QueueUser getByName(Queue queue, String userName)
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
    public QueueUser getById(Queue queue, UUID userId) throws UserNotFoundByIdException {
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
    private QueueUser findOrCreate(Queue queue, ApelleUser user) {
        QueueUser queueUser = QueueUser.findById(user, queue);
        if (queueUser == null) {
            return QueueUser.builder()
                    .queue(queue)
                    .user(user)
                    .role(queueUserRolesService.getDefaultRole())
                    .likesFilled(true)
                    .build();
        }
        return queueUser;
    }

    public void delete(QueueUser user) throws ActionNotPermittedException {
        QueueUser current = getCurrent(user.getQueue());
        if (!current.getPermissions().queueUsers().remove()) {
            throw new ActionNotPermittedException(current.getRole(), "remove user");
        }
        user.delete();
    }

    public short likes(QueueUser user, QueuedSong song) {
        assert user.getQueue().getId().equals(song.getQueue().getId());
        return likes(user.getUser(), song);
    }

    public short likes(ApelleUser user, QueuedSong song) {
        return likes(user.getId(), song);
    }

    public short likes(UUID userId, QueuedSong song) {
        return Likes.givenBy(userId, song);
    }

    public short likes(UUID userId, UUID queueId, UUID songId) {
        return Likes.givenBy(userId, queueId, songId);
    }
}
