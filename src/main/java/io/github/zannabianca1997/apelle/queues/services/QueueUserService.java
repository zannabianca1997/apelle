package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
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
     * @throws QueueNotFoundException The queue does not exist
     */
    public QueueUser getCurrent(UUID queueId) throws QueueNotFoundException {
        return findOrCreate(queueId, usersService.getCurrent());
    }

    /**
     * Get a queue user by name
     * 
     * @param queueId  The queue id
     * @param userName The user name
     * @return The queue user
     * @throws UserNotFoundByNameException The user does not exist
     * @throws QueueNotFoundException      The queue does not exist
     */
    public QueueUser getByName(UUID queueId, String userName)
            throws UserNotFoundByNameException, QueueNotFoundException {
        return findOrCreate(queueId, usersService.get(userName));
    }

    /**
     * Get a queue user by name
     * 
     * @param queueId The queue id
     * @param userId  The user id
     * @return The queue user
     * @throws UserNotFoundByIdException The user does not exist
     * @throws QueueNotFoundException    The queue does not exist
     */
    public QueueUser getById(UUID queueId, UUID userId) throws QueueNotFoundException, UserNotFoundByIdException {
        return findOrCreate(queueId, usersService.get(userId));
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
     * @throws QueueNotFoundException The queue does not exist
     */
    private QueueUser findOrCreate(UUID queueId, ApelleUser user) throws QueueNotFoundException {
        QueueUser queueUser = QueueUser.findById(user.getId(), queueId);
        if (queueUser == null) {
            return QueueUser.builder()
                    .queue(queueService.get(queueId))
                    .user(user)
                    .role(queueUserRolesService.getDefaultRole())
                    .likesFilled(true)
                    .build();
        }
        return queueUser;
    }
}
