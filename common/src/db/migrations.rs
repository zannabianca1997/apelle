use derive_more::From;
use serde::{Deserialize, Serialize};
use snafu::Snafu;
use sqlx::migrate::{AppliedMigration, Migrate, MigrateError, Migration, Migrator};

#[derive(Debug)]
pub struct Migrations {
    /// Base migrations
    pub base: Migrator,
    /// Dev enviroment migrations
    pub environs: phf::Map<&'static str, Migrator>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MigrationsConfigs {
    pub environ: String,
}

impl Default for MigrationsConfigs {
    fn default() -> Self {
        Self {
            environ: if cfg!(debug_assertions) {
                "dev"
            } else {
                "prod"
            }
            .into(),
        }
    }
}

#[derive(Debug, Snafu, From)]
pub enum Error {
    #[snafu(display("Cannot fetch migration list"))]
    FetchMigration { source: MigrateError },

    #[snafu(display("Migration {}_{} was not found in the database", migration.version, migration.description))]
    MigrationMissin { migration: &'static Migration },

    #[snafu(display("Migration {}_{} was changed", migration.version, migration.description))]
    ChecksumMismatch {
        migration: &'static Migration,
        checksum: std::borrow::Cow<'static, [u8]>,
    },
}

impl Migrations {
    pub async fn check(
        &'static self,
        mut db: impl Migrate,
        MigrationsConfigs { environ }: &MigrationsConfigs,
    ) -> Result<(), Error> {
        let env_migrator = self.environs.get(&environ);

        let migrations = self
            .base
            .iter()
            .chain(env_migrator.into_iter().flat_map(Migrator::iter));
        let applied: Vec<AppliedMigration> = db.list_applied_migrations().await?;

        for migration in migrations {
            let Some(applied) = applied
                .iter()
                .find(|applied| applied.version == migration.version)
            else {
                return Err(Error::MigrationMissin { migration });
            };

            if applied.checksum != migration.checksum {
                return Err(Error::ChecksumMismatch {
                    migration,
                    checksum: applied.checksum.clone(),
                });
            }
        }

        Ok(())
    }
}
