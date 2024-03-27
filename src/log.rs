use crate::env::Log as Config;
use anyhow::Result;
use std::str::FromStr;
use tracing_subscriber::{fmt::writer::MakeWriterExt, layer::SubscriberExt};

struct LocalTimer;

impl tracing_subscriber::fmt::time::FormatTime for LocalTimer {
	fn format_time(&self, w: &mut tracing_subscriber::fmt::format::Writer<'_>) -> std::fmt::Result {
		write!(w, "{}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S.%3f"))
	}
}

pub fn init(config: &Config) -> Result<()> {
	let level = tracing::Level::from_str(&config.level)?;
	let file_layer = if config.to_file {
		let appender = tracing_appender::rolling::RollingFileAppender::builder()
			.rotation(tracing_appender::rolling::Rotation::DAILY)
			.filename_suffix("log")
			.build("logs")?;
		let (non_blocking, guard) = tracing_appender::non_blocking(appender);
		std::mem::forget(guard);
		Some(
			tracing_subscriber::fmt::layer()
				.with_writer(non_blocking.with_max_level(level))
				.with_ansi(false)
				.with_level(true)
				.with_thread_names(false)
				.with_target(false)
				.with_line_number(false)
				.with_timer(LocalTimer),
		)
	} else {
		None
	};

	let std_layer = if config.console {
		let (non_blocking, guard) = tracing_appender::non_blocking(std::io::stdout());
		std::mem::forget(guard);
		Some(
			tracing_subscriber::fmt::layer()
				.with_writer(non_blocking.with_max_level(level))
				.with_ansi(true)
				.with_level(true)
				.with_thread_names(false)
				.with_target(false)
				.with_line_number(false)
				.with_timer(LocalTimer),
		)
	} else {
		None
	};

	let subscriber = tracing_subscriber::Registry::default().with(file_layer).with(std_layer);
	tracing::subscriber::set_global_default(subscriber)?;
	Ok(())
}
