package io.github.zannabianca1997.apelle.users.services;

import java.util.UUID;

import io.github.zannabianca1997.apelle.users.exceptions.UserAlreadyExistsException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByIdException;
import io.github.zannabianca1997.apelle.users.exceptions.UserNotFoundByNameException;
import io.github.zannabianca1997.apelle.users.models.ApelleUser;
import io.quarkus.security.identity.SecurityIdentity;
import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class UsersService {

    @Inject
    private SecurityIdentity securityIdentity;

    /**
     * Crate a user
     * 
     * @param user The user to create
     * @return The created user
     * @throws UserAlreadyExistsException The user already exists
     */
    public ApelleUser signup(ApelleUser user) throws UserAlreadyExistsException {
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
    public ApelleUser get(String userName) throws UserNotFoundByNameException {
        ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            throw new UserNotFoundByNameException(userName);
        }
        return user;
    }

    /**
     * Delete a user by name
     * 
     * @param userId The user name
     * @throws UserNotFoundByNameException The user does not exists
     */
    public void delete(String userName) throws UserNotFoundByNameException {
        ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            throw new UserNotFoundByNameException(userName);
        }
        user.delete();
    }

    /**
     * Obtain a user by id
     * 
     * @param userId The user id
     * @return The found user
     * @throws UserNotFoundByNameException The user does not exists
     */
    public ApelleUser get(UUID userId) throws UserNotFoundByIdException {
        ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            throw new UserNotFoundByIdException(userId);
        }
        return user;
    }

    /**
     * Delete a user by id
     * 
     * @param userId The user id
     * @throws UserNotFoundByNameException The user does not exists
     */
    public void delete(UUID userId) throws UserNotFoundByIdException {
        ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            throw new UserNotFoundByIdException(userId);
        }
        user.delete();
    }

    /**
     * Get the used currently logged in
     * 
     * @return The current user
     */
    public ApelleUser getCurrent() {
        String name = securityIdentity.getPrincipal().getName();
        ApelleUser user = ApelleUser.findByName(name);
        return user;
    }

    /**
     * Delete the current user
     */
    public void deleteCurrent() {
        String name = securityIdentity.getPrincipal().getName();
        ApelleUser.findByName(name).delete();
    }
}
