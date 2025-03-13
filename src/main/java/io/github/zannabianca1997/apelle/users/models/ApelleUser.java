package io.github.zannabianca1997.apelle.users.models;

import java.util.UUID;

import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.Table;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import io.quarkus.security.jpa.Password;
import io.quarkus.security.jpa.Roles;
import io.quarkus.security.jpa.UserDefinition;
import io.quarkus.security.jpa.Username;

@Data
@EqualsAndHashCode(callSuper = false)
@AllArgsConstructor
@NoArgsConstructor
@Builder
@Entity
@Table(name = "apelle_user")
@UserDefinition
/// A user
public class ApelleUser extends PanacheEntityBase {
    @Id
    @GeneratedValue(strategy = GenerationType.UUID)
    /// Unique ID of the user
    private UUID id;

    @NonNull
    @Column(nullable = false, unique = true)
    @Username
    /// Username
    private String name;

    @NonNull
    @Column(nullable = false)
    @Password
    /// Hashed password
    private String password;

    @Roles
    /// Comma separated list of roles
    private String roles;

    /**
     * Find a user by name
     * 
     * @param name the username
     * @return The user found, or null if no user was found
     */
    public static ApelleUser findByName(String name) {
        return find("name", name).firstResult();
    }
}
