use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Log {
	#[serde(default = "default_log_level")]
	pub level: String,
	#[serde(default = "default_log_console")]
	pub console: bool,
	#[serde(default)]
	pub to_file: bool,
}
fn default_log_level() -> String {
	"info".into()
}
fn default_log_console() -> bool {
	true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
	pub host: String,
	#[serde(default = "default_database_port")]
	pub port: u32,
	pub username: String,
	pub password: String,
	pub database: String,
	#[serde(default)]
	pub url: Option<String>,
	#[serde(default)]
	pub min_conn: u32,
	#[serde(default = "default_database_max_conn")]
	pub max_conn: u32,
	#[serde(default)]
	pub idle_timeout: Option<u64>,
	pub show_sql: bool,
	pub slow_sql_millis: Option<u64>,
}
fn default_database_port() -> u32 {
	3306
}
fn default_database_max_conn() -> u32 {
	(num_cpus::get() * 2) as u32
}

pub fn init<'de, T: Deserialize<'de>>(
	config_name: Option<&str>,
	prefix: Option<&str>,
) -> anyhow::Result<T> {
	let settings = config::Config::builder()
		.add_source(config::File::with_name(config_name.unwrap_or("config.toml")).required(false))
		.add_source(
			config::Environment::with_prefix(prefix.unwrap_or("starfish"))
				.try_parsing(true)
				.separator("__"),
		)
		.build()?;
	let config: T = settings.try_deserialize()?;
	Ok(config)
}
