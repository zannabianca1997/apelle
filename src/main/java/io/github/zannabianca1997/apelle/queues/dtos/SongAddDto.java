package io.github.zannabianca1997.apelle.queues.dtos;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonSubTypes.Type;

import io.github.zannabianca1997.apelle.queues.dtos.sources.youtube.YoutubeSongAddDto;
import lombok.Data;
import lombok.experimental.SuperBuilder;

/**
 * Fields required to add a song to a queue
 */
@Data
@SuperBuilder
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "kind")
@JsonSubTypes({ @Type(value = YoutubeSongAddDto.class) })
@Schema(description = "Data defining a song to add", oneOf = { YoutubeSongAddDto.class })
public abstract class SongAddDto {

}