package io.github.zannabianca1997.apelle.queues.configs;

import java.util.Map;
import java.util.Optional;
import java.util.Set;

import io.smallrye.config.ConfigMapping;
import io.smallrye.config.WithDefault;
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
            Queue queue();

            public interface Queue {
                @WithDefault("false")
                boolean start();

                @WithDefault("false")
                boolean stop();

                @WithDefault("false")
                boolean next();

                @WithDefault("false")
                boolean like();

                @WithDefault("false")
                boolean enqueue();

                @WithDefault("false")
                boolean remove();

                @WithDefault("false")
                boolean ban();
            }

            QueueUsers queueUsers();

            public interface QueueUsers {
                Optional<Set<String>> grantRoles();

                Optional<Set<String>> removeRoles();

                @WithDefault("false")
                boolean remove();

                @WithDefault("false")
                boolean ban();
            }
        }
    }
}