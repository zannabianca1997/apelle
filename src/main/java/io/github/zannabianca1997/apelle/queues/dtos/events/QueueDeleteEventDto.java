package io.github.zannabianca1997.apelle.queues.dtos.events;

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
@JsonTypeName(QueueDeleteEventDto.JSON_TYPE_NAME)
@Schema(description = """
        Signal that the queue was deleted.

        This message also signal the closure of the event stream.""", properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = { QueueDeleteEventDto.JSON_TYPE_NAME })
}, requiredProperties = { "kind" })
public final class QueueDeleteEventDto extends QueueEventDto {
    final static String JSON_TYPE_NAME = "queue-delete";
}
