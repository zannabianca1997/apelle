package io.github.zannabianca1997.apelle.queues.dtos.sources.youtube;

import org.eclipse.microprofile.openapi.annotations.enums.SchemaType;
import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.eclipse.microprofile.openapi.annotations.media.SchemaProperty;

import com.fasterxml.jackson.annotation.JsonProperty;
import com.fasterxml.jackson.annotation.JsonTypeName;

import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.experimental.SuperBuilder;
import lombok.extern.jackson.Jacksonized;

import io.github.zannabianca1997.apelle.queues.dtos.SongAddDto;
import io.github.zannabianca1997.apelle.queues.dtos.SongKind;

@Data
@EqualsAndHashCode(callSuper = true)
@SuperBuilder
@Jacksonized
@JsonTypeName(SongKind.Constants.YOUTUBE)
@Schema(description = "A song that comes from youtube", properties = {
        @SchemaProperty(name = "kind", type = SchemaType.STRING, enumeration = {
                SongKind.Constants.YOUTUBE })
}, requiredProperties = { "kind" })
public class YoutubeSongAddDto extends SongAddDto {
    @JsonProperty(required = true, value = "video_id")
    @Schema(description = "The video ID")
    private String videoId;
}
