package io.github.zannabianca1997.apelle.queues.configs;

import java.util.Map;

import io.smallrye.config.ConfigMapping;
import jakarta.validation.constraints.PositiveOrZero;

@ConfigMapping(prefix = "apelle.queue.user")
public interface QueueUserRolesConfig {
    String defaultRole();

    String creatorRole();

    String bannedRole();

    Map<String, QueueUserRoleConfig> roles();

    public interface QueueUserRoleConfig {
        @PositiveOrZero
        short maxLikes();

        Permissions permissions();

        public interface Permissions {
            boolean startSong();

            boolean stopSong();

            boolean nextSong();

            boolean likeSong();

            boolean enqueue();
        }
    }
}