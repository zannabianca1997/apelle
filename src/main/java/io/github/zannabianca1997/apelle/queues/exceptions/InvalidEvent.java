package io.github.zannabianca1997.apelle.queues.exceptions;

import jakarta.ws.rs.InternalServerErrorException;
import lombok.experimental.StandardException;

@StandardException
public class InvalidEvent extends InternalServerErrorException {
}