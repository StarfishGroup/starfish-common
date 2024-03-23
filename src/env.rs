use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Log {
    #[serde(default = "default_log_level")]
    pub level: String,
    #[serde(default = "default_log_console")]
    pub console: bool,
    #[serde(default)]
    pub file: bool,
}

fn default_log_level() -> String {
    "info".into()
}

fn default_log_console() -> bool {
    true
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Web {
    #[serde(default = "default_web_bind")]
    pub bind: String,
}
fn default_web_bind() -> String {
    "0.0.0.0:10000".into()
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Orm {
    #[serde(default)]
    pub url: String,
    #[serde(default = "default_orm_max_conn")]
    pub max_conn: u32,
    #[serde(default)]
    pub show_sql: bool,
}

fn default_orm_max_conn() -> u32 {
    num_cpus::get() as u32
}

pub fn init<'de, T: Deserialize<'de>>() -> anyhow::Result<T> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("config.toml").required(false))
        .add_source(
            config::Environment::with_prefix("starfish")
                .try_parsing(true)
                .separator("__"),
        )
        .build()?;
    let config: T = settings.try_deserialize()?;
    Ok(config)
}
