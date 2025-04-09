package io.github.zannabianca1997.apelle.queues.events;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

/**
 * A queue started playing
 */
@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
public final class QueueStartEvent extends QueueEvent {
    private QueueQueryDto state;

    @Override
    public boolean preventsAutoStop() {
        return false;
    }
}
