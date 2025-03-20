package io.github.zannabianca1997.apelle.queues.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.exceptions.QueueNotFoundException;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.github.zannabianca1997.apelle.queues.models.QueueUserRole;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class QueueUserService {

    @Inject
    SecurityIdentity securityIdentity;
    @Inject
    QueueService queueService;

    /**
     * Get the queue user for the current user
     * 
     * @param queueId The queue id
     * @return The queue user
     * @throws QueueNotFoundException The queue does not exist
     */
    public QueueUser getCurrent(UUID queueId) throws QueueNotFoundException {
        ApelleUser user = ApelleUser.findByName(securityIdentity.getPrincipal().getName());
        QueueUser queueUser = QueueUser.findById(user.getId(), queueId);

        if (queueUser == null) {
            queueUser = QueueUser.builder()
                    .queue(queueService.get(queueId))
                    .user(user)
                    .role(QueueUserRole.getDefault())
                    .build();
            queueUser.persist();
        }

        return queueUser;
    }
}
