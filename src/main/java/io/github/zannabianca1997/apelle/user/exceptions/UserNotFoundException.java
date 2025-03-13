package io.github.zannabianca1997.apelle.user.exceptions;

import lombok.Getter;

@Getter
public abstract class UserNotFoundException extends Exception {
    protected UserNotFoundException(String message) {
        super(message);
    }

}
