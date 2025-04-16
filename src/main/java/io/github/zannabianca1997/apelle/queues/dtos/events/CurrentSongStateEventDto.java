package io.github.zannabianca1997.apelle.queues.dtos.events;

import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.media.SchemaProperty;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;

import io.github.zannabianca1997.apelle.queues.dtos.CurrentSongQueryDto;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@JsonTypeName(CurrentSongStateEventDto.JSON_TYPE_NAME)
@Schema(description = """
        An authoritative broadcast of the current song state.

        After receiving this message a client must assume the current song is in the provided state.""", properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = {
                CurrentSongStateEventDto.JSON_TYPE_NAME })
}, requiredProperties = { "kind" })
public final class CurrentSongStateEventDto extends QueueEventDto {
    final static String JSON_TYPE_NAME = "current-song-state";

    @JsonProperty(required = false)
    private CurrentSongQueryDto current;
}
