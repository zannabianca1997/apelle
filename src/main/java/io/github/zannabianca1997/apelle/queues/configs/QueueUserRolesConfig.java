package io.github.zannabianca1997.apelle.queues.configs;

import java.util.Map;
import java.util.Optional;
import java.util.Set;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.smallrye.config.ConfigMapping;
import io.smallrye.config.WithDefault;
import io.smallrye.config.WithName;
import jakarta.validation.constraints.PositiveOrZero;

@ConfigMapping(prefix = "apelle.queue.user.roles")
public interface QueueUserRolesConfig {
    @JsonProperty(value = "default", required = true)
    @WithName("default")
    @Schema(examples = { "VOTER" })
    String default_();

    @JsonProperty(value = "creator", required = true)
    @Schema(examples = { "PLAYER" })
    String creator();

    @JsonProperty(value = "banned", required = true)
    @Schema(examples = { "OBSERVER" })
    String banned();

    @JsonProperty(value = "roles", required = true)
    @Schema(examples = { "[\"PLAYER\", \"VOTER\", \"OBSERVER\"]" })
    default Set<String> rolesNames() {
        return roles().keySet();
    }

    Map<String, QueueUserRoleConfig> roles();

    public interface QueueUserRoleConfig {
        @PositiveOrZero
        short maxLikes();

        Permissions permissions();

        public interface Permissions {
            @JsonProperty(required = true)
            Queue queue();

            @JsonProperty(required = true)
            QueueUsers queueUsers();

            @WithDefault("false")
            @JsonProperty(required = true)
            boolean delete();

            @Schema(name = "QueuePermissions")
            public interface Queue {
                @WithDefault("false")
                @JsonProperty(required = true)
                boolean start();

                @WithDefault("false")
                @JsonProperty(required = true)
                boolean stop();

                @WithDefault("false")
                @JsonProperty(required = true)
                boolean next();

                @WithDefault("false")
                @JsonProperty(required = true)
                boolean like();

                @WithDefault("false")
                @JsonProperty(required = true)
                boolean enqueue();

                @WithDefault("false")
                @JsonProperty(required = true)
                boolean remove();

                @WithDefault("false")
                @JsonProperty(required = true)
                boolean ban();
            }

            @Schema(name = "QueueUsersPermissions")
            public interface QueueUsers {
                @JsonProperty(required = true)
                @Schema(examples = { "[\"PLAYER\", \"VOTER\", \"OBSERVER\"]" })
                Optional<Set<String>> grantRoles();

                @JsonProperty(required = true)
                @Schema(examples = { "[\"VOTER\", \"OBSERVER\"]" })
                Optional<Set<String>> removeRoles();

                @JsonProperty(required = true)
                @WithDefault("false")
                boolean remove();

                @JsonProperty(required = true)
                @WithDefault("false")
                boolean ban();
            }
        }
    }
}