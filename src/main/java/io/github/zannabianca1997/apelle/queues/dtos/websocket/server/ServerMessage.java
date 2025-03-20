package io.github.zannabianca1997.apelle.queues.dtos.websocket.server;

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
@JsonSubTypes({ @Type(value = QueueStateMessage.class) })
@Schema(description = """
        A message from the server.

        The `kind` property discriminates between the different messages.""", oneOf = { QueueStateMessage.class,
        UnknowQueueMessage.class })
public abstract class ServerMessage {

}
