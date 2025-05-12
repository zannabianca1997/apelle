package io.github.zannabianca1997.apelle.users.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.users.exceptions.CannotDeleteUserException;
import io.github.zannabianca1997.apelle.users.exceptions.UserAlreadyExistsException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.github.zannabianca1997.apelle.users.models.ApelleUserRole;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.enterprise.context.ApplicationScoped;

@ApplicationScoped
public class UsersService {
    private final SecurityIdentity securityIdentity;

    public UsersService(final SecurityIdentity securityIdentity) {
        this.securityIdentity = securityIdentity;
    }

    /**
     * Crate a user
     * 
     * @param user The user to create
     * @return The created user
     * @throws UserAlreadyExistsException The user already exists
     */
    public ApelleUser signup(final ApelleUser user) throws UserAlreadyExistsException {
        if (ApelleUser.findByName(user.getName()) != null) {
            throw new UserAlreadyExistsException(user.getName());
        }
        user.persist();
        return user;
    }

    /**
     * Obtain a user by name
     * 
     * @param userName The user name
     * @return The found user
     * @throws UserNotFoundByNameException The user does not exists
     */
    public ApelleUser getByName(final String userName) throws UserNotFoundByNameException {
        final ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            throw new UserNotFoundByNameException(userName);
        }
        return user;
    }

    /**
     * Obtain a user by id
     * 
     * @param userId The user id
     * @return The found user
     * @throws UserNotFoundByNameException The user does not exists
     */
    public ApelleUser getById(final UUID userId) throws UserNotFoundByIdException {
        final ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            throw new UserNotFoundByIdException(userId);
        }
        return user;
    }

    /**
     * Get the used currently logged in
     * 
     * @return The current user
     */
    public ApelleUser getMe() {
        final String name = securityIdentity.getPrincipal().getName();
        final ApelleUser user = ApelleUser.findByName(name);
        return user;
    }

    /**
     * Delete a user by id
     * 
     * @param userId The user id
     * @throws CannotDeleteUserException Cannot delete the user
     */
    public void delete(final ApelleUser user) throws CannotDeleteUserException {
        final ApelleUser deleter = getMe();

        if (!deleter.getRoles().contains(ApelleUserRole.ADMIN) && deleter.getId() != user.getId()) {
            throw new CannotDeleteUserException();
        }

        user.delete();
    }

}
