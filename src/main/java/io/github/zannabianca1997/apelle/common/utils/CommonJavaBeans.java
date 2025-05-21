package io.github.zannabianca1997.apelle.common.utils;

import java.security.SecureRandom;

import jakarta.inject.Singleton;

public class CommonJavaBeans {

    /**
     * Create the random number generator
     * 
     * @return The created random number generator
     */
    @Singleton
    public SecureRandom rng() {
        return new SecureRandom();
    }
}
