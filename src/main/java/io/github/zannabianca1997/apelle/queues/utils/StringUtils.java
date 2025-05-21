package io.github.zannabianca1997.apelle.queues.utils;

import java.security.SecureRandom;

import jakarta.enterprise.context.ApplicationScoped;

@ApplicationScoped
public class StringUtils {
    private final SecureRandom random;

    public StringUtils(final SecureRandom random) {
        this.random = random;
    }

    /**
     * Generate a human readable random string
     * 
     * @param complexity The number of byte of complexity wanted
     * @return The generated string
     */
    public String random(final double complexity, final String alphabet) {
        // Calculate the lenght of the generated string
        final int len = Math.max(1, (int) (complexity * Math.log(256) / Math.log(alphabet.length())));
        final StringBuilder stringBuilder = new StringBuilder(len);
        for (int i = 0; i < len; i++) {
            stringBuilder.append(alphabet.charAt(random.nextInt(alphabet.length())));
        }
        return stringBuilder.toString();
    }
}
