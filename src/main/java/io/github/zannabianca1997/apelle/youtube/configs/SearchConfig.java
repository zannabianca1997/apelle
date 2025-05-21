package io.github.zannabianca1997.apelle.youtube.configs;

import java.time.Duration;

import io.smallrye.config.ConfigMapping;
import io.smallrye.config.WithDefault;
import jakarta.validation.constraints.Max;
import jakarta.validation.constraints.Positive;

/**
 * Configuration for the youtube search api
 */
@ConfigMapping(prefix = "apelle.songs.sources.youtube.search")
public interface SearchConfig {

    /** How many results to ask youtube at the same time */
    @WithDefault("50")
    @Max(value = 50, message = "Youtube support fetching a maximum of 50 results at a time")
    @Positive(message = "At least one value must be fetched")
    int pageSize();

    /**
     * Maximum number of youtube query a user call is allowed to cause
     * 
     * This is usually not a check triggered by the user, as the page size allowed
     * to the frontend is lower than the youtube page size limit
     */
    @WithDefault("1")
    @Positive(message = "At least one query should be allowed")
    Integer queriesAllowedPerRequest();

    /**
     * Expiration time of the youtube search cache.
     * 
     * After this time passed the cached search will be evicted and a new search
     * will be initiated.
     */
    @WithDefault("P1D")
    Duration cacheExpiration();

}
