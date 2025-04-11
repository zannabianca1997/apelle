package io.github.zannabianca1997.apelle.queues.models;

import java.io.Serializable;
import java.sql.PreparedStatement;
import java.sql.ResultSet;
import java.sql.SQLException;
import java.sql.Types;

import org.eclipse.microprofile.openapi.annotations.media.Schema;
import org.hibernate.engine.spi.SharedSessionContractImplementor;
import org.hibernate.usertype.UserType;
import com.fasterxml.jackson.annotation.JsonIgnore;

import io.github.zannabianca1997.apelle.queues.configs.QueueUserRolesConfig.QueueUserRoleConfig;
import io.github.zannabianca1997.apelle.queues.exceptions.RoleDoesNotExistException;
import io.github.zannabianca1997.apelle.queues.services.QueueUserRolesService;
import jakarta.enterprise.inject.spi.CDI;
import lombok.AllArgsConstructor;
import lombok.Getter;
import lombok.NonNull;

/**
 * Role that a user has in a queue
 */
@AllArgsConstructor
public class QueueUserRole {
    @Getter
    @NonNull
    @Schema(examples = { "PLAYER", "VOTER", "OBSERVER" }, required = true)
    private String name;

    @NonNull
    @JsonIgnore
    private QueueUserRoleConfig config;

    @Schema(examples = { "4" }, required = true)
    public short getMaxLikes() {
        return config.maxLikes();
    }

    @Schema(required = true)
    public QueueUserRoleConfig.Permissions getPermissions() {
        return config.permissions();
    }

    public static class Type implements UserType<QueueUserRole> {
        private QueueUserRolesService service;

        public Type() {
            service = CDI.current().select(QueueUserRolesService.class).get();
        }

        @Override
        public int getSqlType() {
            return Types.VARCHAR;
        }

        @Override
        public Class<QueueUserRole> returnedClass() {
            return QueueUserRole.class;
        }

        @Override
        public boolean equals(QueueUserRole x, QueueUserRole y) {
            if (x == null || y == null) {
                return x == null && y == null;
            }
            return x.getName().equals(y.getName());
        }

        @Override
        public int hashCode(QueueUserRole x) {
            return x.hashCode();
        }

        @Override
        public QueueUserRole nullSafeGet(ResultSet rs, int column, SharedSessionContractImplementor session,
                Object owner)
                throws SQLException {
            String name = rs.getString(column);
            if (name == null) {
                return null;
            }
            try {
                return service.getRole(name);
            } catch (RoleDoesNotExistException e) {
                throw new SQLException(
                        String.format("The role `%s` was found in the database, but not in the configurations", name),
                        e);
            }
        }

        @Override
        public void nullSafeSet(PreparedStatement st, QueueUserRole value, int index,
                SharedSessionContractImplementor session) throws SQLException {
            if (value == null) {
                st.setNull(index, Types.VARCHAR);
            } else {
                st.setString(index, value.getName());
            }
        }

        @Override
        public QueueUserRole deepCopy(QueueUserRole value) {
            return value;
        }

        @Override
        public boolean isMutable() {
            return false;
        }

        @Override
        public Serializable disassemble(QueueUserRole value) {
            return value.getName();
        }

        @Override
        public QueueUserRole assemble(Serializable cached, Object owner) {
            try {
                return service.getRole((String) cached);
            } catch (RoleDoesNotExistException e) {
                throw new RuntimeException(String.format("Unknow role made his way into the cache: %s", cached));
            }
        }

        @Override
        public QueueUserRole replace(QueueUserRole detached, QueueUserRole managed, Object owner) {
            return detached;
        }
    }

}
