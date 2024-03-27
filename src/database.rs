use std::time::Duration;

use crate::env;
use anyhow::Result;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};

pub async fn init(config: &env::Database) -> Result<Pool<MySql>> {
	let pool = MySqlPoolOptions::new()
		.idle_timeout(config.idle_timeout.map(|v| Duration::from_secs(v)))
		.min_connections(config.min_conn)
		.max_connections(config.max_conn)
		.connect(&format!(
			"mysql://{}:{}@{}:{}/{}{}",
			config.username,
			config.password,
			config.host,
			config.port,
			config.database,
			match &config.url {
				Some(url) => format!("?{}", url),
				None => "".into(),
			}
		))
		.await?;
	Ok(pool)
}
