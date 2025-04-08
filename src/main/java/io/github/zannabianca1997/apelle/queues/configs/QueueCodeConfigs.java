package io.github.zannabianca1997.apelle.queues.configs;

import io.smallrye.config.ConfigMapping;
import io.smallrye.config.WithDefault;
import jakarta.validation.constraints.Min;
import jakarta.validation.constraints.NotBlank;
import jakarta.validation.constraints.Size;

@ConfigMapping(prefix = "apelle.queue.code")
public interface QueueCodeConfigs {
    @NotBlank
    @Size(min = 2)
    @WithDefault("ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    String alphabet();

    CodeComplexityConfig complexity();

    public interface CodeComplexityConfig {
        @Min(1)
        @WithDefault("3")
        int min();

        @Min(1)
        @WithDefault("1")
        int margin();
    }
}
