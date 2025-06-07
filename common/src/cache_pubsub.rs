use redis::{IntoConnectionInfo as _, RedisError, aio::ConnectionManager};
use reqwest::Url;

/// Connect to the cache service
pub async fn connect(cache_url: Url) -> Result<ConnectionManager, RedisError> {
    let mut conn_info = cache_url.into_connection_info()?;
    if conn_info.redis.protocol != redis::ProtocolVersion::RESP3 {
        tracing::warn!(
            proposed_protocol =? conn_info.redis.protocol,
            "Apelle only supports RESP3 protol, ignoring the one set up and switching to it"
        );
        conn_info.redis.protocol = redis::ProtocolVersion::RESP3;
    }
    let client = redis::Client::open(conn_info)?;
    client.get_connection_manager().await
}
