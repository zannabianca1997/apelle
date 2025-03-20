package io.github.zannabianca1997.apelle;

import org.mapstruct.MapperConfig;
import org.mapstruct.ReportingPolicy;
import org.mapstruct.MappingConstants.ComponentModel;

@MapperConfig(componentModel = ComponentModel.CDI, unmappedTargetPolicy = ReportingPolicy.ERROR)
public interface MappersConfig {
}
