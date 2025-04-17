package io.github.zannabianca1997.apelle.queues.events;

import java.util.UUID;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

/**
 * A song in the queue was liked
 */
@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
public final class QueuedSongDeleteEvent extends QueueEvent {
    /**
     * Id of the song to delete from the queue
     */
    @NonNull
    private UUID deletedId;
}
