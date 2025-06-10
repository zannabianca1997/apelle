use std::collections::HashSet;

use apelle_configs_dtos::QueueConfig;
use itertools::Itertools as _;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ValidateError {
    #[snafu(display("Role {name} is referred but not defined"))]
    UnknownRole { name: String },
    #[snafu(display("Roles [{}] are referred but not defined", names.iter().join(", ")))]
    UnknownRoles { names: HashSet<String> },
}

/// Check if the config is valid and self-consistent
pub fn validate(config: QueueConfig) -> Result<QueueConfig, ValidateError> {
    let mut unknown_roles = HashSet::new();
    for referred_roles in [
        &config.creator_role,
        &config.default_role,
        &config.banned_role,
    ]
    .into_iter()
    .chain(
        config
            .roles
            .values()
            .flat_map(|role| role.can_grant.iter().chain(&role.can_revoke)),
    ) {
        if !config.roles.contains_key(referred_roles) {
            unknown_roles.insert(referred_roles.to_string());
        }
    }

    if !unknown_roles.is_empty() {
        if unknown_roles.len() == 1 {
            return Err(ValidateError::UnknownRole {
                name: unknown_roles.into_iter().next().unwrap(),
            });
        }
        return Err(ValidateError::UnknownRoles {
            names: unknown_roles,
        });
    }

    Ok(config)
}
