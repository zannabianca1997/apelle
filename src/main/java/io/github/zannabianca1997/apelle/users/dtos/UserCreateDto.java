package io.github.zannabianca1997.apelle.users.dtos;

import com.fasterxml.jackson.annotation.JsonProperty;

import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NonNull;

@Data
@EqualsAndHashCode(callSuper = false)
@Builder
/// Data needed to create a user
public class UserCreateDto {
    @NonNull
    @JsonProperty(required = true)
    /// Username
    private String name;

    @NonNull
    @JsonProperty(required = true)
    /// Hashed password
    private String password;
}
