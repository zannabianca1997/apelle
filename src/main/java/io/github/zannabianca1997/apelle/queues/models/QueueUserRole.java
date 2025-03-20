package io.github.zannabianca1997.apelle.queues.models;

import org.eclipse.microprofile.config.ConfigProvider;

/**
 * Roles that a user has in a queue
 */
public enum QueueUserRole {
    ADMIN, VOTER, OBSERVER;

    /**
     * @return The default role for all users
     */
    public static QueueUserRole getDefault() {
        return ConfigProvider.getConfig()
                .getValue("apelle.queue.user.default-role", QueueUserRole.class);
    }

    /**
     * @return The maximum number of likes this user can give
     */
    public short getMaxLikes() {
        return switch (this) {
            case ADMIN -> Short.MAX_VALUE;
            case OBSERVER -> 0;
            case VOTER -> ConfigProvider.getConfig()
                    .getValue(String.format("apelle.queue.user.%s.max-likes", this), Short.class);
        };
    }

    /**
     * @return If this user can control the queue
     */
    public boolean canControlQueue() {
        return switch (this) {
            case ADMIN -> true;
            case OBSERVER -> false;
            case VOTER -> false;
        };
    }

    /**
     * @return If this user can add songs to the queue
     */
    public boolean canEnqueue() {
        return switch (this) {
            case ADMIN -> true;
            case VOTER -> true;
            case OBSERVER -> false;
        };
    }
}
