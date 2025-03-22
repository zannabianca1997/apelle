package io.github.zannabianca1997.apelle.queues.events;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

/**
 * A song in the queue was liked
 */
@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
public final class QueueLikeEvent extends QueueEvent {
    private QueueQueryDto state;
}
