package io.github.zannabianca1997.apelle.users.dtos;

import org.eclipse.microprofile.openapi.annotations.media.Schema;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
@Schema(description = "User creation data")
public class UserCreateDto {
    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Unique username for the user")
    private String name;

    @NonNull
    @JsonProperty(required = true)
    @Schema(description = "Password for the user")
    private String password;
}
