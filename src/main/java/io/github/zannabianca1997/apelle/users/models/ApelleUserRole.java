package io.github.zannabianca1997.apelle.users.models;

import com.fasterxml.jackson.annotation.JsonValue;

import io.quarkus.security.jpa.RolesValue;

public enum ApelleUserRole {
    ADMIN,
    USER;

    @Override
    @RolesValue
    @JsonValue
    public String toString() {
        return super.toString();
    }
}
