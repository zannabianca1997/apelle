package io.github.zannabianca1997.apelle.queues.dtos;

import com.fasterxml.jackson.annotation.JsonSubTypes;
import com.fasterxml.jackson.annotation.JsonTypeInfo;
import com.fasterxml.jackson.annotation.JsonSubTypes.Type;

import lombok.Data;
import lombok.experimental.SuperBuilder;

import io.github.zannabianca1997.apelle.queues.dtos.sources.youtube.YoutubeSongAddDto;

/**
 * Fields required to add a song to a queue
 */
@Data
@SuperBuilder
@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, include = JsonTypeInfo.As.PROPERTY, property = "kind")
@JsonSubTypes({ @Type(YoutubeSongAddDto.class) })
public abstract class SongAddDto {
}
