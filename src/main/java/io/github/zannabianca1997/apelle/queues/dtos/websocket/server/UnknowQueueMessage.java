package io.github.zannabianca1997.apelle.queues.dtos.websocket.server;

import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.media.SchemaProperty;

import com.fasterxml.jackson.annotation.JsonTypeName;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@JsonTypeName("unknow-queue")
@Schema(description = """
        The queue id is invalid.

        Either the queue does not exist, it was cancelled.

        After this message the socket will be closed.
        """, properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = {
                "unknow-queue" }) })
public final class UnknowQueueMessage extends ServerMessage {
    private String queueId;
}
