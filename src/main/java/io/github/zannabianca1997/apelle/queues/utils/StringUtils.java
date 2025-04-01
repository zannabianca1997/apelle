package io.github.zannabianca1997.apelle.queues.utils;

import java.security.SecureRandom;

import jakarta.enterprise.context.ApplicationScoped;
import jakarta.inject.Inject;

@ApplicationScoped
public class StringUtils {

    @Inject
    SecureRandom secureRandom;

    public String randomHumanReadable(String complexity) {
        
    }
}
