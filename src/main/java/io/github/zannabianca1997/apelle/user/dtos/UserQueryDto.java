package io.github.zannabianca1997.apelle.user.dtos;

import java.util.UUID;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
/// Data about a user
public class UserQueryDto {
    @NonNull
    @JsonProperty(required = true)
    /// Unique ID of the user
    private UUID id;

    @NonNull
    @JsonProperty(required = true)
    /// Username
    private String name;

    @JsonProperty(required = true)
    /// Comma separated list of roles
    private String roles;
}
