use apelle_common::db::migrations::{Migrations, MigrationsConfigs};
use futures::{FutureExt, future::BoxFuture};
use snafu::{ResultExt, Snafu};
use sqlx::{
    PgPool,
    migrate::{MigrationSource, Migrator},
};

use crate::config::Config;

pub static MIGRATIONS: &[&Migrations] = &[
    &::apelle_songs::MIGRATIONS,
    &::apelle_users::MIGRATIONS,
    &::apelle_songs_youtube::MIGRATIONS,
    &::apelle_configs::MIGRATIONS,
    &::apelle_queues::MIGRATIONS,
];

mod config;

#[derive(Debug, Snafu)]
pub enum MainError {
    Connect { source: sqlx::Error },
    Migrate { source: sqlx::migrate::MigrateError },
}

#[derive(Debug)]
struct AllMigrations<'e> {
    environ: &'e str,
}
impl<'e> MigrationSource<'e> for AllMigrations<'e> {
    fn resolve(
        self,
    ) -> BoxFuture<'e, Result<Vec<sqlx::migrate::Migration>, sqlx::error::BoxDynError>> {
        let mut migrations = vec![];

        for m in MIGRATIONS {
            migrations.extend(m.base.iter().cloned());

            if let Some(env) = m.environs.get(self.environ) {
                migrations.extend(env.iter().cloned());
            }
        }

        migrations.sort_by_key(|m| m.version);

        futures::future::ready(Ok(migrations)).boxed()
    }
}

pub async fn app(
    Config {
        db_url,
        migrate: MigrationsConfigs { environ },
    }: Config,
) -> Result<(), MainError> {
    // Create all migrations

    let migrator = Migrator::new(AllMigrations { environ: &environ })
        .await
        .expect("The migration are loaded infallibly");

    for migration in migrator.iter() {
        tracing::info!(migration.version, %migration.description, "Loaded migration")
    }

    let db = PgPool::connect(db_url.as_str())
        .await
        .context(ConnectSnafu)?;

    migrator.run(&db).await.context(MigrateSnafu)
}
