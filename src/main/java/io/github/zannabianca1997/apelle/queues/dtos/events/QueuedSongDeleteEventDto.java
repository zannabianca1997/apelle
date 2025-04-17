package io.github.zannabianca1997.apelle.queues.dtos.events;

import java.util.UUID;

import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.media.SchemaProperty;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@JsonTypeName(QueuedSongDeleteEventDto.JSON_TYPE_NAME)
@Schema(description = """
        Signal the removal of a single song.

        Nothing else is changed. The song should be removed mantaining the order of the others.""", properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = {
                QueuedSongDeleteEventDto.JSON_TYPE_NAME })
}, requiredProperties = { "kind" })
public final class QueuedSongDeleteEventDto extends QueueEventDto {
    final static String JSON_TYPE_NAME = "queued-song-delete";

    @NonNull
    @JsonProperty(value = "deleted_id", required = true)
    @Schema(description = "The song to delete")
    private UUID deletedId;
}
