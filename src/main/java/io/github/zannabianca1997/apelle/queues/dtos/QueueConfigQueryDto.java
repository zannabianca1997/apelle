package io.github.zannabianca1997.apelle.queues.dtos;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.NonNull;
import lombok.extern.jackson.Jacksonized;

@Data
@Builder
@Jacksonized
public class QueueConfigQueryDto {
    @Schema(description = "If true, the user will like every song they add to the queue")
    @JsonProperty(required = true)
    private boolean autolike;

    @NonNull
    @Schema(description = "The role that will be given to users when they join the queue")
    @JsonProperty(required = true, value = "default_role")
    private UUID defaultRole;

    @NonNull
    @Schema(description = "The role that will be given to the user that created the queue")
    @JsonProperty(required = true, value = "creator_role")
    private UUID creatorRole;

    @NonNull
    @Schema(description = "The role that will be given to users when they are banned from the queue")
    @JsonProperty(required = true, value = "banned_role")
    private UUID bannedRole;
}
