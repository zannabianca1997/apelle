package io.github.zannabianca1997.apelle.queues.resources;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertThrows;

import org.hibernate.exception.ConstraintViolationException;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Tag;
import org.junit.jupiter.api.Test;

import io.quarkus.test.junit.QuarkusTest;
import io.github.zannabianca1997.apelle.queues.models.Queue;
import jakarta.transaction.Transactional;

@QuarkusTest
@Tag("apiChecks")
public class QueueNameConflictTest {
    @BeforeEach
    @Transactional
    void deleteAllQueues() {
        Queue.deleteAll();
    }

    @Test
    @Transactional
    void conflictShouldThrown() {
        Queue.builder()
                .code("code")
                .build()
                .persist();

        final var ex = assertThrows(ConstraintViolationException.class, () -> {
            Queue.builder()
                    .code("code")
                    .build()
                    .persistAndFlush();
        });

        assertEquals(Queue.CODE_UNIQUE_CONSTRAINT_NAME, ex.getConstraintName());
    }
}
