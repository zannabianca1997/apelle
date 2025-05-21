package io.github.zannabianca1997.apelle.queues.models;

import java.net.URL;

/**
 * A thumbnail
 */
public interface Thumbnail {
    /**
     * @return The thumbnail width
     */
    int getWidth();

    /**
     * @return The thumbnail height
     */
    int getHeight();

    /**
     * @return The public URL where the thumbnail can be found
     */
    URL getUrl();
}
