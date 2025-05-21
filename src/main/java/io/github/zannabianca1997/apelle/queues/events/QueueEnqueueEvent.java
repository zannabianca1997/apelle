package io.github.zannabianca1997.apelle.queues.events;

import java.util.List;

import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongShortQueryDto;
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
public final class QueueEnqueueEvent extends QueueEvent {
    private List<QueuedSongShortQueryDto> queuedSongs;
}
