use crate::env::Orm as Config;
use anyhow::Result;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};

pub async fn init(config: &Config) -> Result<DatabaseConnection> {
    let mut opt = ConnectOptions::new(&config.url);
    opt.max_connections(config.max_conn);
    opt.sqlx_logging(config.show_sql);
    let db = Database::connect(opt).await?;
    Ok(db)
}
