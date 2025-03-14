package io.github.zannabianca1997.apelle.youtube;

import java.net.URI;
import java.net.URISyntaxException;

/**
 * Collection of URIs to give to the user
 */
public final class ConstantUris {
    /**
     * Youtube watch endpoint
     */
    public final static URI WATCH = knowUri("https://www.youtube.com/watch");

    /**
     * Convert a string into a uri
     * 
     * @param uri The uri string
     * @return The parsed URI
     */
    private static URI knowUri(String uri) {
        try {
            return new URI(uri);
        } catch (URISyntaxException e) {
            throw new RuntimeException(e);
        }
    }
}
