package io.github.zannabianca1997.apelle.queues.dtos.events;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonSubTypes.Type;
import com.fasterxml.jackson.annotation.JsonTypeInfo;

import lombok.Data;
import lombok.experimental.SuperBuilder;

/**
 * A message sent by the server
 */
@Data
@SuperBuilder
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "kind")
@JsonSubTypes({
        @Type(value = QueueStateEventDto.class),
        @Type(value = QueueDeleteEventDto.class),
        @Type(value = CurrentSongStateEventDto.class),
        @Type(value = QueuedSongsStateEventDto.class)
})
@Schema(description = """
        A message from the server.

        The `kind` property discriminates between the different messages.""", oneOf = {
        QueueStateEventDto.class,
        QueueDeleteEventDto.class,
        CurrentSongStateEventDto.class,
        QueuedSongsStateEventDto.class
}, requiredProperties = { "kind" })
public abstract class QueueEventDto {

}
