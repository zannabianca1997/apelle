package io.github.zannabianca1997.apelle.common.dtos;

import java.net.URL;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import lombok.Builder;
import lombok.Data;
import lombok.NonNull;

/**
 * A thumbnail
 */
@Data
@Builder
@Schema(description = "A thumbnail for a song")
public class ThumbnailQueryDto {
    @Schema(description = "The thumbnail width", required = true)
    private int width;

    @Schema(description = "The thumbnail height", required = true)
    private int height;

    @Schema(description = "The public URL where the thumbnail can be found", required = true)
    @NonNull
    private URL url;
}
