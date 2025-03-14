package io.github.zannabianca1997.apelle.queues.dtos.sources.youtube;

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
public class YoutubeSongAddDto extends SongAddDto {
    @JsonProperty(required = true, value = "video_id")
    private String videoId;
}
