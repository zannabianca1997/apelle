package io.github.zannabianca1997.apelle.queues.dtos.websocket.server;

import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.media.SchemaProperty;

import com.fasterxml.jackson.annotation.JsonTypeName;

import io.github.zannabianca1997.apelle.queues.dtos.QueueQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@JsonTypeName("queue-state")
@Schema(description = """
        An authoritative broadcast of the queue state.

        After receiving this message a client must assume the queue is in the provided state.""", properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = {
                "queue-state" }) })
public final class QueueStateMessage extends ServerMessage {
    private QueueQueryDto queue;
}
