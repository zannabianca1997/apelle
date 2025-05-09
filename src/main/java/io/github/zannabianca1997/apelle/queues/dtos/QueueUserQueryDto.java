package io.github.zannabianca1997.apelle.queues.dtos;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import io.github.zannabianca1997.apelle.users.dtos.UserQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@Schema(description = "Data about a user of a queue")
public class QueueUserQueryDto extends UserQueryDto {
    @NonNull
    @JsonProperty(value = "queue_role", required = true)
    @Schema(description = "Role of the user in the queue. Fetch the actual permissions from `/queues/roles/{id}`")
    private UUID queueRole;

    @JsonProperty(required = true)
    @Schema(description = "Number of likes given in the queue")
    private short likes;

    @JsonProperty(value = "max_likes", required = true)
    @Schema(description = "Maximum number of likes that can be given")
    private short maxLikes;

}
