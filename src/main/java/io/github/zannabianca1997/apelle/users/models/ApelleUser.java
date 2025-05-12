package io.github.zannabianca1997.apelle.users.models;

import java.util.ArrayList;
import java.util.Collection;
import java.util.Set;
import java.util.UUID;

import org.hibernate.annotations.OnDelete;
import org.hibernate.annotations.OnDeleteAction;

import io.github.zannabianca1997.apelle.queues.models.Likes;
import io.github.zannabianca1997.apelle.queues.models.QueueUser;
import io.quarkus.elytron.security.common.BcryptUtil;
import io.quarkus.hibernate.orm.panache.PanacheEntityBase;
import jakarta.persistence.CascadeType;
import jakarta.persistence.Column;
import jakarta.persistence.Entity;
import jakarta.persistence.EnumType;
import jakarta.persistence.Enumerated;
import jakarta.persistence.GeneratedValue;
import jakarta.persistence.GenerationType;
import jakarta.persistence.Id;
import jakarta.persistence.OneToMany;
import jakarta.persistence.Table;
import lombok.AccessLevel;
import lombok.Builder;
import lombok.EqualsAndHashCode;
import lombok.Getter;
import lombok.Setter;
import lombok.ToString;
import lombok.NoArgsConstructor;
import lombok.NonNull;
import lombok.Singular;
import io.quarkus.security.jpa.Password;
import io.quarkus.security.jpa.Roles;
import io.quarkus.security.jpa.UserDefinition;
import io.quarkus.security.jpa.Username;

@Getter
@Setter
@ToString
@EqualsAndHashCode(callSuper = false, of = { "id" })
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
    @Column(nullable = false)
    @Enumerated(EnumType.STRING)
    /// Comma separated list of roles
    private Set<ApelleUserRole> roles;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "user")
    /// The queues this user voted on
    private Collection<QueueUser> queues;

    @NonNull
    @OnDelete(action = OnDeleteAction.CASCADE)
    @OneToMany(cascade = CascadeType.ALL, mappedBy = "user")
    /// The likes given by this user
    private Collection<Likes> likes;

    /**
     * Find a user by name
     * 
     * @param name the username
     * @return The user found, or null if no user was found
     */
    public static ApelleUser findByName(final String name) {
        return find("name", name).firstResult();
    }

    @Builder
    public ApelleUser(final @NonNull String name, final @NonNull String password,
            final @Singular Set<ApelleUserRole> roles) {
        super();
        this.name = name;
        this.password = BcryptUtil.bcryptHash(password);
        this.roles = roles;
        this.queues = new ArrayList<>();
        this.likes = new ArrayList<>();
    }
}
