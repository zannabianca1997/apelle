package io.github.zannabianca1997.apelle.queues.utils;

import java.security.SecureRandom;

import org.eclipse.microprofile.config.inject.ConfigProperty;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

@ApplicationScoped
public class StringUtils {

    @Inject
    SecureRandom random;

    @ConfigProperty(name = "apelle.codes.alphabet", defaultValue = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    @NotBlank
    @Size(min = 2)
    String alphabet;

    /**
     * Generate a human readable random string
     * 
     * @param complexity The number of byte of complexity wanted
     * @return The generated string
     */
    public String randomHumanReadable(double complexity) {
        // Calculate the lenght of the generated string
        int len = Math.max(1, (int) (complexity * Math.log(256) / Math.log(alphabet.length())));
        StringBuilder stringBuilder = new StringBuilder(len);
        for (int i = 0; i < len; i++) {
            stringBuilder.append(alphabet.charAt(random.nextInt(alphabet.length())));
        }
        return stringBuilder.toString();
    }
}
