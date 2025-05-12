package io.github.zannabianca1997.apelle.queues.dtos.events;

import java.util.List;

import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.media.SchemaProperty;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;

import io.github.zannabianca1997.apelle.queues.dtos.QueuedSongShortQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@JsonTypeName(QueuedSongsStateEventDto.JSON_TYPE_NAME)
@Schema(description = """
        An authoritative broadcast of the queued songs state.

        After receiving this message a client must assume the queued songs are in the provided state.""", properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = {
                QueuedSongsStateEventDto.JSON_TYPE_NAME })
}, requiredProperties = { "kind" })
public final class QueuedSongsStateEventDto extends QueueEventDto {
    public final static String JSON_TYPE_NAME = "queued-songs-state";

    @NonNull
    @JsonProperty(value = "queue", required = true)
    @Schema(description = "The songs in the queue")
    private List<QueuedSongShortQueryDto> queuedSongs;
}
