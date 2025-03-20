package io.github.zannabianca1997.apelle.queues.events;

import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonTypeInfo;

import lombok.Data;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;

/**
 * An event relative to a queue
 */
@Data
@SuperBuilder
@JsonTypeInfo(use = JsonTypeInfo.Id.CLASS, include = JsonTypeInfo.As.PROPERTY, property = "event")
public abstract sealed class QueueEvent permits QueueEnqueueEvent, QueuePlayEvent, QueueStopEvent {
    /**
     * Id of the affected queue
     */
    @NonNull
    private UUID queueUuid;
}
