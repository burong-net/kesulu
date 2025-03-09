use sqlx::Result;

use sqlx::{sqlite::SqliteConnectOptions, Error, SqlitePool};
use tracing::instrument;
use tracing::info;


#[instrument]
pub async fn db_connect() -> Result<()> {
    let options = SqliteConnectOptions::new()
        .filename("kesulu.db")
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await.unwrap();

    info!("{:?}", pool);

    Ok(())
}