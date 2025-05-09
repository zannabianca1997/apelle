package io.github.zannabianca1997.apelle.queues.roles.dtos;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.extern.jackson.Jacksonized;

@Data
@Builder
@Jacksonized
public class QueueUserRoleQueuePermissionsQueryDto {
    @JsonProperty(required = true)
    private boolean start;
    @JsonProperty(required = true)
    private boolean stop;
    @JsonProperty(required = true)
    private boolean next;
    @JsonProperty(required = true)
    private boolean like;
    @JsonProperty(required = true)
    private boolean enqueue;
    @JsonProperty(required = true)
    private boolean remove;
    @JsonProperty(required = true)
    private boolean ban;
}
