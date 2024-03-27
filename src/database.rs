use std::{str::FromStr, time::Duration};

use crate::env;
use anyhow::Result;
use sqlx::{
	mysql::{MySqlConnectOptions, MySqlPoolOptions},
	ConnectOptions, MySql, Pool,
};

pub async fn init(config: &env::Database) -> Result<Pool<MySql>> {
	let mut opt = MySqlConnectOptions::from_str(&format!(
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
	))?;
	if config.show_sql {
		opt = opt.log_statements(log::LevelFilter::Info);
		if let Some(slow_sql_millis) = config.slow_sql_millis {
			opt = opt.log_slow_statements(
				log::LevelFilter::Warn,
				Duration::from_millis(slow_sql_millis),
			);
		}
	} else {
		opt = opt.disable_statement_logging();
	}

	let pool = MySqlPoolOptions::new()
		.idle_timeout(config.idle_timeout.map(|v| Duration::from_secs(v)))
		.min_connections(config.min_conn)
		.max_connections(config.max_conn)
		.connect_with(opt)
		.await?;
	Ok(pool)
}
