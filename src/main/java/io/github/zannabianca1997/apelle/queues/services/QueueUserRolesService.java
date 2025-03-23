package io.github.zannabianca1997.apelle.queues.services;

import io.github.zannabianca1997.apelle.queues.configs.QueueUserRolesConfig;
import io.github.zannabianca1997.apelle.queues.exceptions.RoleDoesNotExistException;
import io.github.zannabianca1997.apelle.queues.models.QueueUserRole;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import lombok.NonNull;

@ApplicationScoped
public class QueueUserRolesService {

    @Inject
    private QueueUserRolesConfig config;

    public QueueUserRole getDefaultRole() {
        try {
            return getRole(config.defaultRole());
        } catch (RoleDoesNotExistException e) {
            throw new RuntimeException("The default role was not configured", e);
        }
    }

    public QueueUserRole getCreatorRole() {
        try {
            return getRole(config.creatorRole());
        } catch (RoleDoesNotExistException e) {
            throw new RuntimeException("The creator role was not configured", e);
        }
    }

    public QueueUserRole getBannedRole() {
        try {
            return getRole(config.bannedRole());
        } catch (RoleDoesNotExistException e) {
            throw new RuntimeException("The banned role was not configured", e);
        }
    }

    public QueueUserRole getRole(@NonNull String name) throws RoleDoesNotExistException {
        var roleConfig = config.roles().get(name);
        if (roleConfig == null) {
            throw new RoleDoesNotExistException(name);
        }
        return new QueueUserRole(name, roleConfig);
    }

}