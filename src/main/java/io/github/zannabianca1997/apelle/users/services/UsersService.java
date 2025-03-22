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

    public ApelleUser signup(ApelleUser user) throws UserAlreadyExistsException {
        if (ApelleUser.findByName(user.getName()) != null) {
            throw new UserAlreadyExistsException(user.getName());
        }
        user.persist();
        return user;
    }

    public ApelleUser get(String userName) throws UserNotFoundByNameException {
        ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            throw new UserNotFoundByNameException(userName);
        }
        return user;
    }

    public void delete(String userName) throws UserNotFoundByNameException {
        ApelleUser user = ApelleUser.findByName(userName);
        if (user == null) {
            throw new UserNotFoundByNameException(userName);
        }
        user.delete();
    }

    public ApelleUser get(UUID userId) throws UserNotFoundByIdException {
        ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            throw new UserNotFoundByIdException(userId);
        }
        return user;
    }

    public void delete(UUID userId) throws UserNotFoundByIdException {
        ApelleUser user = ApelleUser.findById(userId);
        if (user == null) {
            throw new UserNotFoundByIdException(userId);
        }
        user.delete();
    }

    public ApelleUser getCurrent() {
        String name = securityIdentity.getPrincipal().getName();
        ApelleUser user = ApelleUser.findByName(name);
        return user;
    }

    public void deleteCurrent() {
        String name = securityIdentity.getPrincipal().getName();
        ApelleUser.findByName(name).delete();
    }
}
