package io.github.zannabianca1997.apelle.queues.events;

import java.util.UUID;

import io.github.zannabianca1997.apelle.queues.dtos.CurrentSongQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

/**
 * A queue was stopped
 */
@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
public final class QueueStopEvent extends QueueEvent {
    private CurrentSongQueryDto state;
    private UUID playerStateId;
}
