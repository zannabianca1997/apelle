use std::iter::once;

use apelle_common::Reporter;
use textwrap_macros::unfill;
use tokio::sync::mpsc::{Receiver, Sender, error::SendError};

/// Worker that updates the source table when sources are seen or used
#[derive(Debug, Clone)]
pub struct SeenSourcesWorker {
    queue: Sender<String>,
}

impl SeenSourcesWorker {
    pub async fn new(db: sqlx::PgPool, bufsize: usize) -> Self {
        let (queue, receiver) = tokio::sync::mpsc::channel(bufsize);
        tokio::spawn(worker(db, receiver, bufsize));
        Self { queue }
    }

    pub async fn seen_urn(&self, source_urn: String) {
        if let Err(SendError(urn)) = self.queue.send(source_urn).await {
            tracing::warn!(%urn, "Queue is closed, cannot update source");
        };
    }
}

async fn worker(db: sqlx::Pool<sqlx::Postgres>, mut receiver: Receiver<String>, bufsize: usize) {
    let mut buffer = Vec::with_capacity(bufsize);

    while receiver.recv_many(&mut buffer, bufsize).await > 0 {
        if let Err(e) = sqlx::query(
            unfill!(
                "
                        UPDATE source
                        SET last_heard = NOW()
                        FROM UNNEST($1::text[]) AS updates (urn)
                        WHERE source.urn = updates.urn
                        "
            )
            .trim_ascii(),
        )
        .bind(&buffer)
        .execute(&db)
        .await
        {
            tracing::error!("Failed to update seen sources: {}", Reporter(e));
        };

        buffer.clear();
    }
}
