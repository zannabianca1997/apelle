package io.github.zannabianca1997.apelle.users.models;

import java.util.Collection;
import java.util.Set;
import java.util.UUID;

import org.hibernate.annotations.JdbcTypeCode;
import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;
import org.hibernate.type.SqlTypes;

import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.quarkus.elytron.security.common.BcryptUtil;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.OneToMany;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.Data;
import lombok.EqualsAndHashCode;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Singular;
import io.quarkus.security.jpa.Password;
import io.quarkus.security.jpa.Roles;
import io.quarkus.security.jpa.UserDefinition;
import io.quarkus.security.jpa.Username;

@Data
@EqualsAndHashCode(callSuper = false)
@NoArgsConstructor(access = AccessLevel.PROTECTED)
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

    @NonNull
    @Roles
    @JdbcTypeCode(SqlTypes.JSON)
    @Column(nullable = false)
    /// Comma separated list of roles
    private Set<ApelleUserRole> roles;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "link.user")
    /// The queues this user voted on
    private Set<QueueUser> queues;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "link.user")
    /// The likes given by this user
    private Collection<Likes> likes;

    /**
     * Find a user by name
     * 
     * @param name the username
     * @return The user found, or null if no user was found
     */
    public static ApelleUser findByName(String name) {
        return find("name", name).firstResult();
    }

    @Builder
    public ApelleUser(@NonNull String name, @NonNull String password, @Singular Set<ApelleUserRole> roles) {
        super();
        this.name = name;
        this.password = BcryptUtil.bcryptHash(password);
        this.roles = roles;
    }
}
