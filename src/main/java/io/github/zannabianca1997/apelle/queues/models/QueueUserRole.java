package io.github.zannabianca1997.apelle.queues.models;

import org.eclipse.microprofile.config.ConfigProvider;

/**
 * Roles that a user has in a queue
 */
public enum QueueUserRole {
    ADMIN, VOTER, OBSERVER;

    short getMaxLikes() {
        return switch (this) {
            case ADMIN -> Short.MAX_VALUE;
            case OBSERVER -> 0;
            case VOTER -> ConfigProvider.getConfig()
                    .getValue(String.format("apelle.queue.max-likes.%s", this), Short.class);
        };
    }
}
